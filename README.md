# Face Blur Detection – Real-time Face Detection System 

**Authors:**
Rayhan Marcello Ananda Purnomo | Nurhafid Sudarianto | Maulida Rahmayanti | Muhammad Rakha Randika | Amisha Nabila Putri Wiguna | Faqih Chairul Anam
---

## Abstract

Face Blur Detection merupakan aplikasi yang dibuat untuk membantu menjaga privasi dengan cara mendeteksi dan memburamkan wajah secara otomatis melalui kamera secara real-time. Aplikasi ini dibangun menggunakan Rust sebagai backend dan React (Vite) sebagai frontend, dengan dukungan framework Axum dan runtime Tokio agar sistem dapat berjalan cepat dan efisien dalam menangani banyak proses secara bersamaan. Untuk mendeteksi wajah, sistem menggunakan model AI YOLO11n melalui ONNX Runtime, sementara OpenCV digunakan untuk mengambil gambar dari webcam. Pendekatan functional programming diterapkan agar kode lebih rapi, mudah dikelola, dan mengurangi kesalahan. Hasilnya, aplikasi dapat bekerja secara stabil, responsif, dan cukup akurat dalam melakukan deteksi wajah secara real-time.
---

## Introduction

Berikut adalah beberapa alasan utama di balik pengembangan aplikasi Face Blur Detection, baik dari sisi masalah yang diangkat maupun teknologi yang digunakan.

### 1. What problem does your application solve?
Seiring meningkatnya penggunaan kamera di berbagai aplikasi, masalah privasi menjadi semakin penting. Banyak pengguna tidak menyadari bahwa wajah mereka dapat direkam, disimpan, atau disalahgunakan. Aplikasi ini dibuat untuk melindungi privasi dengan cara memburamkan wajah secara otomatis saat kamera digunakan, sehingga wajah tidak terlihat secara langsung dan risiko penyalahgunaan data dapat dikurangi.

### 2. Why did you choose Rust?
| No | Alasan                      | Penjelasan Singkat                                                                                     |
| -- | --------------------------- | ------------------------------------------------------------------------------------------------------ |
| 1  | **Performa Tinggi**         | Rust mampu berjalan cepat seperti C/C++ sehingga cocok untuk pemrosesan video dan AI secara real-time. |
| 2  | **Keamanan Memori**         | Rust mencegah error seperti *null pointer* dan *memory leak* tanpa perlu garbage collector.            |
| 3  | **Concurrency Aman**        | Rust mendukung pemrosesan paralel yang aman dan mencegah *data race*.                                  |
| 4  | **Cocok untuk AI & Vision** | Performa Rust ideal untuk inferensi model AI dan pemrosesan gambar.                                    |
| 5  | **Stabil & Andal**          | Aplikasi berjalan lebih stabil untuk penggunaan jangka panjang.                                        |
| 6  | **Integrasi Mudah**         | Mudah terhubung dengan OpenCV, ONNX Runtime, dan library modern lainnya.                               |
| 7  | **Komunitas Berkembang**    | Dukungan komunitas yang aktif dan dokumentasi yang lengkap.                                            |


### Why integrate functional programming concepts?
Konsep functional programming membantu membuat kode lebih rapi, mudah dipahami, dan lebih aman dari error. Pendekatan ini sangat membantu saat mengelola data real-time yang kompleks agar alur program tetap jelas dan terkontrol.

### What makes your solution unique or interesting?
Keunikannya terletak pada penggabungan Rust, AI real-time, dan functional programming dalam satu sistem. Selain itu, penggunaan model YOLO11n yang ringan namun tetap akurat membuat aplikasi ini efisien dan relevan untuk penggunaan nyata.
---

## Background & Concepts
Dengan semakin berkembangnya teknologi kamera dan kecerdasan buatan, penggunaan sistem pendeteksi wajah kini menjadi hal yang umum di berbagai aplikasi. Namun, kemudahan ini juga membawa risiko terhadap privasi, karena wajah merupakan data pribadi yang sensitif. Masih banyak sistem yang mampu mengenali wajah, tetapi belum banyak yang benar-benar fokus melindungi identitas penggunanya. Oleh karena itu, penting untuk memahami konsep dasar dan teknologi yang digunakan dalam aplikasi Face Blur Detection, mulai dari cara kerja deteksi wajah hingga pemrosesan secara real-time. Bagian ini membahas konsep-konsep utama yang menjadi dasar dalam membangun sistem pemburaman wajah secara otomatis.

1. Face Detection
Face detection adalah proses mengenali apakah ada wajah dalam gambar atau video. Dalam aplikasi ini, sistem menggunakan deteksi wajah untuk menemukan posisi wajah secara otomatis sebelum efek blur diberikan. Inilah bagian utama yang memungkinkan aplikasi melindungi privasi pengguna secara langsung.

2. Real-Time Processing
Real-time processing berarti sistem bekerja secara langsung tanpa jeda yang terasa. Saat wajah muncul di kamera, blur akan diterapkan seketika, sehingga pengguna tidak merasakan keterlambatan dan aplikasi tetap terasa responsif.

3. AI Model (YOLO11n)
YOLO11n adalah model AI yang digunakan untuk mengenali wajah dengan cepat. Model ini dipilih karena ringan, tidak membutuhkan banyak resource, tetapi tetap mampu memberikan hasil deteksi yang cukup akurat untuk penggunaan real-time.

4. ONNX Runtime
ONNX Runtime digunakan sebagai mesin untuk menjalankan model AI di backend. Teknologi ini membantu agar proses inferensi berjalan lebih cepat dan stabil, sehingga aplikasi tetap lancar saat digunakan.

5. OpenCV
OpenCV berperan dalam mengambil gambar dari webcam dan mengolah setiap frame video sebelum diproses lebih lanjut oleh sistem deteksi. Library ini mempermudah pengolahan gambar dasar seperti membaca dan memanipulasi frame secara real-time.

6. Rust Backend & React Frontend
Rust digunakan di sisi backend karena cepat, aman, dan andal, sedangkan React (Vite) digunakan untuk menampilkan tampilan aplikasi yang interaktif dan mudah digunakan. Kombinasi ini membuat sistem kuat di balik layar dan nyaman di sisi pengguna.

7. Functional Programming Concept
Pendekatan functional programming membantu pengembangan sistem menjadi lebih tertata. Dengan konsep ini, kode lebih mudah dipahami, lebih stabil, dan tidak mudah menimbulkan error, terutama saat menangani data yang terus berubah dari kamera.

### Technology Stack
| Kategori             | Teknologi              | Fungsi                                                          |
| -------------------- | ---------------------- | --------------------------------------------------------------- |
| Backend              | Rust                   | Bahasa utama backend untuk performa tinggi dan keamanan sistem. |
| Backend Framework    | Axum                   | Membangun API dan routing backend.                              |
| Runtime Async        | Tokio                  | Menjalankan proses asynchronous dan concurrent.                 |
| AI Engine            | ONNX Runtime           | Menjalankan model AI dengan cepat dan efisien.                  |
| Computer Vision      | OpenCV                 | Mengambil dan memproses gambar dari webcam.                     |
| AI Model             | YOLO11n                | Mendeteksi wajah secara cepat dan akurat.                       |
| Frontend             | React                  | Membangun tampilan aplikasi interaktif.                         |
| Frontend Tooling     | Vite                   | Mempercepat proses development frontend.                        |
| Programming Approach | Functional Programming | Memastikan kode rapi, aman, dan minim error.                    |
| Architecture         | Async Architecture     | Menjaga aplikasi tetap responsif saat menangani banyak proses.  |
| Serialization        | Sarde                  | Mengubah struktur data Rust ke JSON dan sebaliknya.             |


### Konsep Functional Programming Dalam Sistem
| Konsep FP                  | Implementasi Dalam Proyek                                                                                                                                          |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Pure Functions**         | Fungsi seperti `preprocess()` dan `postprocess()` digunakan untuk transformasi frame/video tanpa mengubah state luar, sehingga hasilnya konsisten dan mudah diuji. |
| **Immutability**           | Frame yang di-capture dan hasil deteksi wajah tidak diubah secara langsung, melainkan selalu menghasilkan data baru untuk langkah selanjutnya.                     |
| **Pattern Matching**       | Error handling menggunakan `Result<T, AppError>` dan ekspresi `match` untuk menangani kondisi sukses atau gagal secara eksplisit.                                  |
| **Higher-Order Functions** | Fungsi seperti `.map()` dan `.filter()` digunakan untuk memproses dan memanipulasi hasil deteksi wajah dengan cara yang modular dan bersih.                        |
| **Composition**            | Pipeline utama dibangun dari rangkaian fungsi: **Capture → Preprocess → Inference → Postprocess**, sehingga alur data jelas dan kode lebih modular.                |

Dengan ini aplikasi bisa menangani ratusan request detection serentak tanpa bottleneck.
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
Proyek Face Blur Detection membuktikan bahwa Rust bisa digunakan untuk membuat sistem deteksi wajah yang cepat, aman, dan andal, dengan kode yang rapi dan minim error berkat Functional Programming. Sistem ini mampu mendeteksi wajah secara real-time dengan efisien menggunakan YOLO11n dan OpenCV. Ke depannya, aplikasi ini bisa dikembangkan lebih jauh dengan fitur blur otomatis, multi-model, streaming real-time, cloud deployment, dan GPU acceleration, menjadikannya solusi modern yang stabil dan mudah dikembangkan untuk melindungi privasi pengguna.
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

