use crate::config::Device;
use crate::scanner::{scan_file, ScanResult};
use std::path::Path;
use walkdir::WalkDir;

pub fn batch_scan(dir: &Path, devices: &[Device]) -> Vec<ScanResult> {
    let mut results = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                match ext.to_lowercase().as_str() {
                    "mp3" | "m4a" | "aac" | "mp4" | "wav" | "aif" | "aiff" | "flac" | "fla" => {
                        results.push(scan_file(path, devices));
                    }
                    _ => {}
                }
            }
        }
    }
    results
}
