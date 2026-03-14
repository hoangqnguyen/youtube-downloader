use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

static PROGRESS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\[download\]\s+([\d.]+)%\s+of\s+~?\s*([\d.]+\s*\S+)\s+at\s+([\d.]+\s*\S+)\s+ETA\s+(\S+)",
    )
    .unwrap()
});

static DESTINATION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[download\] Destination: (.+)").unwrap()
});

static MERGER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[Merger\]|Merging formats").unwrap()
});

static ALREADY_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[download\] .+ has already been downloaded").unwrap()
});

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgressEvent {
    pub percent: f32,
    pub size: String,
    pub speed: String,
    pub eta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ParsedLine {
    Progress(ProgressEvent),
    Destination(String),
    Merging,
    AlreadyDownloaded,
    Other(String),
}

pub fn parse_line(line: &str) -> ParsedLine {
    if let Some(caps) = PROGRESS_RE.captures(line) {
        let percent = caps[1].parse::<f32>().unwrap_or(0.0);
        return ParsedLine::Progress(ProgressEvent {
            percent,
            size: caps[2].trim().to_string(),
            speed: caps[3].trim().to_string(),
            eta: caps[4].to_string(),
        });
    }
    if let Some(caps) = DESTINATION_RE.captures(line) {
        return ParsedLine::Destination(caps[1].trim().to_string());
    }
    if MERGER_RE.is_match(line) {
        return ParsedLine::Merging;
    }
    if ALREADY_RE.is_match(line) {
        return ParsedLine::AlreadyDownloaded;
    }
    ParsedLine::Other(line.to_string())
}
