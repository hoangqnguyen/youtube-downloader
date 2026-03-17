use crate::binaries;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaylistEntry {
    pub url: String,
    pub title: String,
    pub id: String,
    pub duration: Option<f64>,
    pub thumbnail: Option<String>,
}

#[tauri::command]
pub async fn expand_playlist(
    url: String,
    cookie_browser: String,
) -> Result<Vec<PlaylistEntry>, String> {
    let ytdlp_path = binaries::bin_path("yt-dlp");

    let mut args = vec![
        "--flat-playlist".to_string(),
        "--dump-json".to_string(),
        "--no-warnings".to_string(),
    ];
    if !cookie_browser.is_empty() && cookie_browser != "none" {
        args.extend(["--cookies-from-browser".to_string(), cookie_browser]);
    }
    args.push(url);

    let mut cmd = Command::new(&ytdlp_path);
    cmd.args(&args);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let output = cmd
        .output()
        .await
        .map_err(|e| format!("Failed to run yt-dlp: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp error: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
            let id = val["id"].as_str().unwrap_or("").to_string();
            let title = val["title"]
                .as_str()
                .unwrap_or("Unknown Title")
                .to_string();
            let webpage_url = val["webpage_url"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("https://www.youtube.com/watch?v={}", id));
            let duration = val["duration"].as_f64();
            let thumbnail = val["thumbnail"].as_str().map(|s| s.to_string());

            if !id.is_empty() {
                entries.push(PlaylistEntry {
                    url: webpage_url,
                    title,
                    id,
                    duration,
                    thumbnail,
                });
            }
        }
    }

    Ok(entries)
}
