// Mock detection model for demo
use crate::errors::Result;

pub struct YoloModel;

impl YoloModel {
    pub fn load<P: AsRef<std::path::Path>>(_path: P) -> Result<Self> {
        println!("Mock YOLO model loaded (no real inference)");
        Ok(YoloModel)
    }

    pub fn run(&self, _input: &[u8]) -> Result<Vec<Detection>> {
        // Mock detections: return 2-4 random face boxes
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let num_faces = rng.gen_range(2..=4);
        
        let detections = (0..num_faces)
            .map(|i| Detection {
                x: rng.gen_range(50.0..450.0),
                y: rng.gen_range(50.0..350.0),
                width: rng.gen_range(80.0..150.0),
                height: rng.gen_range(100.0..180.0),
                confidence: rng.gen_range(0.7..0.95),
                class_id: 0, // Face class
            })
            .collect();
        
        Ok(detections)
    }
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub confidence: f32,
    pub class_id: i32,
}

pub type SharedModel = YoloModel;
