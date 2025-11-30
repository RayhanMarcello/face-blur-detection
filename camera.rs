// Mock camera implementation for demo
use crate::errors::{AppError, Result};

pub struct CameraSource;

impl CameraSource {
    pub fn new(_index: i32, _width: f64, _height: f64) -> Result<Self> {
        println!("Mock camera initialized (no real webcam access)");
        Ok(CameraSource)
    }

    pub fn capture(&self) -> Result<Vec<u8>> {
        // Return mock image data (simulates 640x480 RGB frame)
        // In a real scenario, this would capture from webcam
        Ok(vec![0u8; 640 * 480 * 3])
    }
}

pub type SharedCamera = CameraSource;
