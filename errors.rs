use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Webcam capture failed: {0}")] 
    CaptureFailed(String),
    #[error("Detection failed: {0}")] 
    DetectionFailed(String),
    #[error("Invalid image input: {0}")] 
    InvalidImage(String),
    #[error("Internal error: {0}")] 
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl From<image::ImageError> for AppError {
    fn from(e: image::ImageError) -> Self { AppError::InvalidImage(e.to_string()) }
}
