use crate::ytdlp::progress::{parse_line, ParsedLine};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadRequest {
    pub job_id: String,
    pub url: String,
    pub output_dir: String,
    pub audio_only: bool,
    pub resolution: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct JobEvent {
    pub job_id: String,
    #[serde(flatten)]
    pub payload: JobEventPayload,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind", content = "data")]
pub enum JobEventPayload {
    Progress {
        percent: f32,
        size: String,
        speed: String,
        eta: String,
    },
    Merging,
    Done {
        success: bool,
        error: Option<String>,
    },
}

pub async fn run_download(app: AppHandle, req: DownloadRequest) -> Result<(), String> {
    let args = build_args(&req);

    let sidecar = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Failed to find yt-dlp sidecar: {e}"))?
        .args(&args);

    let (mut rx, _child) = sidecar
        .spawn()
        .map_err(|e| format!("Failed to spawn yt-dlp: {e}"))?;

    let mut error_lines: Vec<String> = Vec::new();

    while let Some(event) = rx.recv().await {
        use tauri_plugin_shell::process::CommandEvent;
        match event {
            CommandEvent::Stdout(bytes) => {
                let line = String::from_utf8_lossy(&bytes).into_owned();
                let line = line.trim();
                match parse_line(line) {
                    ParsedLine::Progress(p) => {
                        let _ = app.emit(
                            "job-event",
                            JobEvent {
                                job_id: req.job_id.clone(),
                                payload: JobEventPayload::Progress {
                                    percent: p.percent,
                                    size: p.size,
                                    speed: p.speed,
                                    eta: p.eta,
                                },
                            },
                        );
                    }
                    ParsedLine::Merging => {
                        let _ = app.emit(
                            "job-event",
                            JobEvent {
                                job_id: req.job_id.clone(),
                                payload: JobEventPayload::Merging,
                            },
                        );
                    }
                    ParsedLine::AlreadyDownloaded => {
                        let _ = app.emit(
                            "job-event",
                            JobEvent {
                                job_id: req.job_id.clone(),
                                payload: JobEventPayload::Done {
                                    success: true,
                                    error: None,
                                },
                            },
                        );
                        return Ok(());
                    }
                    _ => {}
                }
            }
            CommandEvent::Stderr(bytes) => {
                let line = String::from_utf8_lossy(&bytes).into_owned();
                let line = line.trim().to_string();
                if !line.is_empty() {
                    error_lines.push(line);
                }
            }
            CommandEvent::Terminated(status) => {
                let success = status.code == Some(0);
                let error = if success {
                    None
                } else {
                    Some(error_lines.join("\n"))
                };
                let _ = app.emit(
                    "job-event",
                    JobEvent {
                        job_id: req.job_id.clone(),
                        payload: JobEventPayload::Done { success, error },
                    },
                );
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn build_args(req: &DownloadRequest) -> Vec<String> {
    let mut args: Vec<String> = vec![
        "--newline".into(),
        "--no-colors".into(),
        "--no-playlist".into(),
        "--windows-filenames".into(),
        "-o".into(),
        format!("{}/%(title)s.%(ext)s", req.output_dir),
    ];

    if req.audio_only {
        args.extend([
            "--extract-audio".into(),
            "--audio-format".into(),
            "mp3".into(),
            "--audio-quality".into(),
            "0".into(),
        ]);
    } else {
        let format = match req.resolution.as_str() {
            "best" | "" => "bestvideo+bestaudio/best".into(),
            res => format!(
                "bestvideo[height<={}]+bestaudio/best[height<={}]",
                res, res
            ),
        };
        args.extend(["--format".into(), format, "--merge-output-format".into(), "mp4".into()]);
    }

    // Embed thumbnail and metadata
    args.extend([
        "--embed-thumbnail".into(),
        "--add-metadata".into(),
    ]);

    args.push(req.url.clone());
    args
}
