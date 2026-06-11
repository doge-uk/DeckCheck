use crate::scanner::ScanResult;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Report {
    files: Vec<FileReport>,
}

#[derive(Serialize)]
pub struct FileReport {
    path: String,
    format: Option<String>,
    bit_depth: Option<u16>,
    sample_rate: Option<u32>,
    bitrate: Option<u32>,
    encoding: Option<String>,
    compatibility: HashMap<String, CompatibilityStatus>,
}

#[derive(Serialize)]
pub struct CompatibilityStatus {
    compatible: bool,
    reasons: Vec<String>,
}

pub fn generate_report(results: &[ScanResult]) -> String {
    let mut file_reports = Vec::new();

    for result in results {
        let mut compatibility = HashMap::new();
        for (device_name, device_result) in &result.device_results {
            compatibility.insert(
                device_name.clone(),
                CompatibilityStatus {
                    compatible: device_result.compatible,
                    reasons: device_result.failures.clone(),
                },
            );
        }

        file_reports.push(FileReport {
            path: result.file_path.to_string_lossy().to_string(),
            format: result.detected_properties.format.clone(),
            bit_depth: result.detected_properties.bit_depth,
            sample_rate: result.detected_properties.sample_rate,
            bitrate: result.detected_properties.bitrate,
            encoding: result.detected_properties.encoding.clone(),
            compatibility,
        });
    }

    let report = Report {
        files: file_reports,
    };

    serde_json::to_string_pretty(&report).unwrap_or_else(|e| {
        format!("{{\"error\": \"Failed to serialize results: {}\"}}", e)
    })
}
