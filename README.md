# Face Blur Detection – Real-time Face Detection System 
**A Functional Programming Approach with Rust**

**Authors:**  
Rayhan Marcello Ananda Purnomo | Nurhafid Sudarianto | Maulida Rahmayanti | Muhammad Rakha Randika | Amisha Nabila Putri Wiguna | Faqih Chairul Anam

---

## Abstract

**Face Blur Detection** adalah aplikasi deteksi wajah real-time yang dibangun menggunakan **Rust** sebagai Backend dan **React (Vite)** sebagai Frontend dengan pendekatan **functional programming**. Backend dikembangkan menggunakan framework **Axum** dan runtime asynchronous **Tokio**, memungkinkan sistem menangani request secara concurrent dan async. Sistem menggunakan **YOLO11n** melalui **ONNX Runtime** untuk inferensi machine learning dan **OpenCV** untuk capture webcam.

---

## Introduction

Aplikasi ini dirancang untuk menyelesaikan permasalahan utama pada sistem deteksi wajah umumnya yaitu:

1. **Lambatnya processing** deteksi wajah pada sistem tradisional
2. **Keterbatasan concurrency** dalam menangani multiple request deteksi
3. Dibutuhkan sistem modern dengan arsitektur **aman, efisien, dan scalable**

### Mengapa Rust?

| Alasan | Penjelasan |
|--------|------------|
| **Memory Safety** | Zero-cost abstractions tanpa garbage collector, mencegah memory leaks |
| **High Concurrency** | Tokio async runtime untuk handle multiple detection requests |
| **Functional Friendly** | Mendukung paradigma pemrograman fungsional (immutability, pattern matching) |
| **Native Performance** | Setara C/C++ untuk ML inference pipeline |

### Tujuan Utama

Memberikan sistem deteksi wajah yang **cepat, scalable, dan aman**  
Menyediakan API detection yang dapat di-integrate dengan aplikasi lain  
Mengaplikasikan paradigma **Functional Programming** dalam implementasi sistem  

---

## Background & Concepts

### Technology Stack

| Komponen | Teknologi |
|----------|-----------|
| **Backend** | Rust + Axum |
| **Frontend** | React (Vite) + Tailwind CSS |
| **Runtime Async** | Tokio |
| **ML Inference** | ONNX Runtime |
| **Computer Vision** | OpenCV |
| **Serialization** | Serde |
| **Model** | YOLO11n (lightweight) |

### Konsep Functional Programming Dalam Sistem

| Konsep FP | Implementasi Dalam Proyek |
|-----------|---------------------------|
| **Pure Functions** | `preprocess()`, `postprocess()` - transformasi image tanpa side effects |
| **Immutability** | Frame capture dan detections tidak di-mutate, selalu return new data |
| **Pattern Matching** | Error handling dengan `Result<T, AppError>` dan `match` expressions |
| **Higher-Order Functions** | `.map()`, `.filter()` untuk transformasi detection results |
| **Composition** | Pipeline: Capture → Preprocess → Inference → Postprocess |

Dengan ini aplikasi bisa menangani **ratusan request detection serentak** tanpa bottleneck.

---

## Source Code Overview

### Struktur Folder Backend

```
face-blur-detection/

face-blur-app/
│
├── backend/
│   ├── Cargo.toml
│   ├── models/
│   │   └── face_detector.onnx        # model ONNX yang sudah kamu training
│   └── src/
│       ├── main.rs                   # entry utama axum
│       │
│       ├── routes/
│       │   ├── mod.rs
│       │   └── face_blur.rs          # endpoint POST /face-blur
│       │
│       ├── services/
│       │   ├── mod.rs
│       │   ├── detector.rs           # inference ONNX + post-processing
│       │   └── blur.rs               # implementasi blur wajah
│       │
│       ├── utils/
│       │   ├── mod.rs
│       │   └── image_convert.rs      # konversi gambar ke tensor & sebaliknya
│       │
│       └── state.rs                  # state global (model ONNX, konfigurasi)
└── README.md
```

---

## File Utama

### `src/main.rs`

Main point aplikasi yang menjalankan:

- Tokio async runtime menggunakan `#[tokio::main]`
- Load YOLO11 model dengan ONNX Runtime
- Initialize webcam dengan OpenCV
- CORS middleware configuration untuk development
- Router dari semua module
- Jalankan Server di `127.0.0.1:3000`

**Snippet Code:**
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting YOLO11 Face Detection Server...");

    // Load model
    let model = YoloModel::load("yolo11n.onnx")?;
    
    // Initialize camera
    let camera = CameraSource::new(0, 640.0, 480.0)?;
    
    let state = AppState::new(camera, model);

    // Build router with CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = api_router(std::sync::Arc::new(state)).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

---

### Models Layer

#### `src/errors.rs`

Membuat custom error types dengan **thiserror**:

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Webcam capture failed: {0}")] 
    CaptureFailed(String),
    
    #[error("Model inference failed: {0}")] 
    InferenceFailed(String),
    
    #[error("Invalid image input: {0}")] 
    InvalidImage(String),
    
    #[error("Internal error: {0}")] 
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

**Functional Programming:**
- Type-safe error handling dengan `Result<T, E>`
- Pattern matching untuk error branching
- Automatic conversion dari OpenCV/ONNX errors

---

#### `src/state.rs`

Shared application state (immutable):

```rust
pub struct AppState {
    pub camera: Arc<SharedCamera>,
    pub model: Arc<SharedModel>,
}

impl AppState {
    pub fn new(camera: SharedCamera, model: SharedModel) -> Self {
        Self { 
            camera: Arc::new(camera), 
            model: Arc::new(model) 
        }
    }
}
```

**Functional Programming:**
- Immutable state dengan `Arc` (Atomic Reference Counting)
- Thread-safe sharing tanpa locks

---

### Business Logic Layer

#### `src/camera.rs`

Webcam operations:

```rust
pub struct CameraSource {
    inner: Arc<Mutex<VideoCapture>>,
}

impl CameraSource {
    pub fn new(index: i32, width: f64, height: f64) -> Result<Self> {
        let mut cam = VideoCapture::new(index, CAP_ANY)?;
        if !cam.is_opened()? { 
            return Err(AppError::CaptureFailed("Unable to open webcam".into())); 
        }
        cam.set(CAP_PROP_FRAME_WIDTH, width)?;
        cam.set(CAP_PROP_FRAME_HEIGHT, height)?;
        Ok(Self { inner: Arc::new(Mutex::new(cam)) })
    }

    pub fn capture(&self) -> Result<Mat> {
        let mut frame = Mat::default();
        self.inner.lock().unwrap().read(&mut frame)?;
        if frame.empty() { 
            return Err(AppError::CaptureFailed("Empty frame".into())); 
        }
        Ok(frame)
    }
}
```

**Functional Programming:**
- Pure function: `capture()` tidak modify internal state
- Error handling dengan `Result` monad

---

#### `src/model.rs`

ONNX model inference:

```rust
pub struct YoloModel {
    session: Session,
}

impl YoloModel {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(path.as_ref())?;
        Ok(Self { session })
    }

    pub fn run(&self, input: &Array<f32, IxDyn>) -> Result<Array<f32, IxDyn>> {
        let outputs = self.session.run(ort::inputs!("images" => input.view())?)?;
        let output = outputs["output0"].try_extract_tensor::<f32>()?;
        Ok(output.view().to_owned())
    }
}
```

**Functional Programming:**
- Immutable model instance
- Pure inference function

---

#### `src/inference.rs`

Image preprocessing & postprocessing:

```rust
pub fn preprocess(frame: &Mat) -> Result<Array<f32, IxDyn>> {
    let rgb = cvt_color(frame, COLOR_BGR2RGB)?;
    let resized = resize(&rgb, Size::new(640, 640))?;
    let normalized = resized.convert_to(CV_32F, 1.0 / 255.0)?;
    
    // Transform HWC → CHW
    let array = transform_to_nchw(normalized)?;
    Ok(array)
}

pub fn postprocess(
    output: Array<f32, IxDyn>,
    conf_threshold: f32,
) -> Result<Vec<Detection>> {
    let detections = (0..output.shape()[2])
        .filter_map(|i| {
            let conf = max_confidence(&output, i);
            if conf > conf_threshold {
                Some(Detection {
                    bbox: extract_bbox(&output, i),
                    confidence: conf,
                })
            } else {
                None
            }
        })
        .collect();
    
    Ok(detections)
}
```

**Functional Programming:**
- Pure transformations
- `.filter_map()` untuk functional iteration
- No mutations, semua return new data

---

### Routes Layer (API Endpoints)

#### `src/routes.rs`

API endpoints:

```rust
pub fn api_router(state: SharedState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/detect", get(detect))
        .with_state(state)
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn detect(State(state): State<SharedState>) -> Result<Json<Value>> {
    let frame = state.camera.capture()?;
    let input = preprocess(&frame)?;
    let output = state.model.run(&input)?;
    let detections = postprocess(output, 0.5)?;
    
    Ok(Json(json!({
        "success": true,
        "num_faces": detections.len(),
        "detections": detections
    })))
}
```

**Functional Programming:**
- Functional pipeline: Capture → Preprocess → Inference → Postprocess
- Error propagation dengan `?` operator
- Immutable request handling

---

## API Endpoints

### `GET /api/health`

Health check endpoint.

**Response:**
```json
{
  "status": "ok"
}
```

---

### `GET /api/detect`

Capture frame dari server webcam dan deteksi wajah.

**Response:**
```json
{
  "success": true,
  "num_faces": 2,
  "detections": [
    {
      "bbox": [120.5, 80.3, 250.7, 220.8],
      "confidence": 0.92
    },
    {
      "bbox": [340.2, 95.1, 470.8, 230.4],
      "confidence": 0.87
    }
  ]
}
```

**Fields:**
- `bbox`: `[x1, y1, x2, y2]` koordinat bounding box
- `confidence`: Skor confidence (0.0 - 1.0)

---

## Setup & Run

### Prerequisites

1. **Rust** (rustup + cargo)
2. **Node.js** v22+
3. **MSVC C++ Build Tools** (Windows)
4. **YOLO11n Model** (`yolo11n.onnx`)

### Installation

```powershell
# Clone repository
git clone https://github.com/RayhanMarcello/face-blur-detection.git
cd face-blur-detection

# Install Rust dependencies & build backend
cargo build --release

# Install frontend dependencies
cd frontend
npm install
npm run dev
```

### Running

**Terminal 1 - Backend:**
```powershell
cargo run
# Server: http://127.0.0.1:3000
```

**Terminal 2 - Frontend:**
```powershell
cd frontend
npm run dev
# Frontend: http://localhost:5173
```

---

## Screenshots

| Tampilan | Status |
|----------|--------|
| API Health Check | ✅ |
| Real-time Detection UI | ✅ |
| Detection Response JSON | ✅ |
| Multi-face Detection | ✅ |

---

## Conclusion

Projek ini menunjukkan bahwa **Rust** dapat digunakan secara efektif untuk membangun layanan **face detection** yang memiliki kebutuhan:

✅ **Cepat & aman** pada sistem concurrency yang tinggi  
✅ Menerapkan paradigma **Functional Programming** dengan konsisten  
✅ **Type-safe** error handling dan memory management  
✅ **Zero-cost abstractions** untuk ML inference pipeline  

### Functional Programming Benefits

- **Immutability** → Thread-safe tanpa locks eksplisit
- **Pure functions** → Predictable, testable, composable
- **Pattern matching** → Exhaustive error handling
- **Higher-order functions** → Elegant data transformations

### Future Enhancements

🔮 Ke depannya, fitur projek ini dapat dikembangkan menjadi:

- **Blur detection** untuk mengaburkan wajah terdeteksi
- **Multi-model support** (face recognition, emotion detection)
- **WebSocket** untuk real-time streaming
- **Batch processing** untuk video files
- **Cloud deployment** dengan containerization (Docker)
- **GPU acceleration** dengan CUDA support

---

## Why C++ Needed? 🤔

Backend Rust ini membutuhkan **C++ Build Tools** karena:

1. **OpenCV** → Library C++ untuk computer vision
2. **ONNX Runtime** → C++ inference engine
3. **FFI Bindings** → Rust wrapper memanggil C++ native code
4. **Linking** → `link.exe` untuk compile native dependencies

**Compilation Pipeline:**
```
Rust Code → Cargo Build → FFI Bindings (C++) → MSVC Linker → Binary
```

Ini adalah trade-off: kita dapat **performance native C++** dengan **safety Rust**.

---

## License

MIT License - Feel free to use and modify! 🚀

