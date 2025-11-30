// Mock inference pipeline for demo
use crate::errors::Result;
use crate::model::Detection;
use serde_json::json;

pub fn preprocess(_frame: &[u8]) -> Result<Vec<u8>> {
    // In mock, just pass through
    Ok(vec![])
}

pub fn postprocess(detections: Vec<Detection>) -> Result<Vec<serde_json::Value>> {
    // Convert Detection structs to JSON format for API response
    let results: Vec<_> = detections
        .into_iter()
        .map(|det| {
            json!({
                "bbox": [det.x, det.y, det.x + det.width, det.y + det.height],
                "confidence": det.confidence,
                "class_id": det.class_id
            })
        })
        .collect();
    
    Ok(results)
}
