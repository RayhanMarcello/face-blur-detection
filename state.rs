use crate::camera::SharedCamera;
use crate::model::SharedModel;
use std::sync::Arc;

pub struct AppState {
    pub camera: Arc<SharedCamera>,
    pub model: Arc<SharedModel>,
}

impl AppState {
    pub fn new(camera: SharedCamera, model: SharedModel) -> Self {
        Self { camera: Arc::new(camera), model: Arc::new(model) }
    }
}

pub type SharedState = Arc<AppState>;
