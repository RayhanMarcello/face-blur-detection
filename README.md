# **Face Blur Detection – Real-time Privacy Protection System 🔒**

_A Functional Programming Approach with Rust & React_

**Author:**<br/>
Rayhan Marcello

---

## **Abstract**

**Face Blur Detection** adalah aplikasi privacy protection yang secara otomatis mendeteksi dan mengaburkan wajah dalam gambar dan video real-time. Dibangun menggunakan **Rust** sebagai Backend (framework **Axum** + runtime **Tokio**) dan **React (Vite)** sebagai Frontend, dengan pendekatan **functional programming**. Backend saat ini berjalan dalam **mock mode** untuk kemudahan development di Windows, sementara frontend menggunakan **face-api.js** untuk deteksi wajah akurat di browser.

---

## **Introduction**

Aplikasi ini dirancang untuk menyelesaikan permasalahan utama dalam perlindungan privasi visual:

1. **Otomatisasi blur wajah** untuk konten sensitif
2. **Processing real-time** dengan latency rendah
3. Dibutuhkan sistem modern dengan arsitektur **aman, efisien, dan scalable**

### Mengapa Rust?

| Alasan | Penjelasan |
|--------|------------|
| **Memory Safety** | Zero-cost abstractions tanpa garbage collector |
| **High Concurrency** | Tokio async runtime untuk handle multiple requests |
| **Functional Friendly** | Mendukung paradigma pemrograman fungsional |
| **Type Safety** | Strong typing mencegah runtime errors |

### Tujuan Utama

✅ Memberikan sistem blur wajah yang **cepat, scalable, dan aman**  
✅ Menyediakan API detection yang dapat di-integrate  
✅ Mengaplikasikan paradigma **Functional Programming**  
✅ Client-side processing untuk privasi maksimal  

---

## **Background & Concepts**

### Technology Stack

| Komponen | Teknologi |
|----------|-----------|
| **Backend** | Rust + Axum |
| **Frontend** | React (Vite) + Tailwind CSS |
| **Runtime Async** | Tokio |
| **Face Detection** | face-api.js (TinyFaceDetector) |
| **Serialization** | Serde |
| **CORS** | tower-http |

### Konsep Functional Programming Dalam Sistem

| Konsep FP | Implementasi Dalam Proyek |
|-----------|---------------------------|
| **Pure Functions** | Detection handlers tidak mutate state |
| **Immutability** | `AppState` wrapped dalam `Arc` untuk thread-safe sharing |
| **Pattern Matching** | Error handling dengan `Result<T, AppError>` dan `match` |
| **Higher-Order Functions** | `.map()`, `.filter()` untuk transformasi data |
| **Composition** | Pipeline: Upload → Detect → Blur → Download |

Dengan ini aplikasi bisa menangani **ratusan request serentak** tanpa bottleneck.

---

## **Source Code Overview**

### Struktur Folder

```
face-blur-detection/
├── backend/
│   ├── src/
│   │   ├── main.rs                  # Entry point + server bootstrap
│   │   ├── errors.rs                # Custom error types
│   │   ├── state.rs                 # Shared application state
│   │   ├── routes.rs                # API endpoints routing
│   │   ├── camera.rs                # Mock camera (demo detections)
│   │   ├── model.rs                 # Mock YOLO model
│   │   └── inference.rs             # Detection pipeline
│   │
│   └── Cargo.toml                   # Rust dependencies
│
├── frontend/
│   ├── src/
│   │   ├── App.jsx                  # Main app component
│   │   │
│   │   ├── hooks/
│   │   │   └── useImageProcessor.jsx    # face-api.js logic
│   │   │
│   │   └── components/
│   │       ├── upload/
│   │       │   ├── ImageUpload.jsx
│   │       │   ├── ImagePreview.jsx
│   │       │   ├── RealtimeDetection.jsx
│   │       │   └── DownloadResult.jsx
│   │       ├── processing/
│   │       │   ├── BlurSlider.jsx
│   │       │   └── ProcessSettings.jsx
│   │       └── results/
│   │           └── ProcessStats.jsx
│   │
│   ├── public/
│   │   └── models/                  # face-api.js model files
│   │
│   └── package.json
│
└── README.md
```

---

## **Backend Architecture**

### **src/main.rs**

Main point aplikasi yang menjalankan:

- Tokio async runtime menggunakan `#[tokio::main]`
- Initialize mock model & camera
- CORS middleware configuration
- Router setup
- Server listening di `127.0.0.1:3000`

**Code Snippet:**
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Face Blur Detection Server (Mock Mode)...");

    let model = YoloModel::new_mock();
    let camera = CameraSource::new_mock();
    let state = AppState::new(camera, model);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = api_router(Arc::new(state)).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("✅ Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}
```

**Functional Programming:**
- Immutable state management
- Error propagation dengan `?` operator
- Pure configuration functions

---

### **src/errors.rs**

Custom error handling dengan **thiserror**:

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Detection failed: {0}")]
    DetectionFailed(String),
    
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DetectionFailed(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ImageError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}
```

**Functional Programming:**
- Type-safe error handling
- Pattern matching untuk error branching
- Automatic HTTP response conversion

---

### **src/state.rs**

Shared application state (immutable):

```rust
pub struct AppState {
    pub camera: Arc<CameraSource>,
    pub model: Arc<YoloModel>,
}

impl AppState {
    pub fn new(camera: CameraSource, model: YoloModel) -> Self {
        Self {
            camera: Arc::new(camera),
            model: Arc::new(model),
        }
    }
}
```

**Functional Programming:**
- Immutable state dengan `Arc`
- Thread-safe sharing tanpa locks eksplisit

---

### **src/routes.rs**

API endpoints:

```rust
pub fn api_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/detect", get(detect))
        .with_state(state)
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn detect(State(state): State<Arc<AppState>>) -> Result<Json<Value>, AppError> {
    let frame = state.camera.capture()?;
    let detections = state.model.detect(&frame)?;
    
    Ok(Json(json!({
        "success": true,
        "num_faces": detections.len(),
        "detections": detections,
        "mock": true
    })))
}
```

**Functional Programming:**
- Pure route handlers
- Functional pipeline: Capture → Detect → Serialize
- Error propagation dengan `?`

---

### **src/camera.rs** & **src/model.rs**

Mock implementations untuk demo:

```rust
// camera.rs
pub struct CameraSource;

impl CameraSource {
    pub fn new_mock() -> Self {
        Self
    }
    
    pub fn capture(&self) -> Result<Vec<u8>, AppError> {
        // Return mock frame data
        Ok(vec![0u8; 640 * 480 * 3])
    }
}

// model.rs
pub struct YoloModel;

impl YoloModel {
    pub fn new_mock() -> Self {
        Self
    }
    
    pub fn detect(&self, _frame: &[u8]) -> Result<Vec<Detection>, AppError> {
        // Return random mock detections
        let mut rng = rand::thread_rng();
        let detections = (0..rng.gen_range(1..4))
            .map(|_| Detection {
                bbox: [
                    rng.gen_range(50.0..300.0),
                    rng.gen_range(50.0..200.0),
                    rng.gen_range(350.0..600.0),
                    rng.gen_range(250.0..450.0),
                ],
                confidence: rng.gen_range(0.7..0.95),
                class_id: 0,
            })
            .collect();
        Ok(detections)
    }
}
```

---

## **Frontend Architecture**

### **src/hooks/useImageProcessor.jsx**

Core logic untuk face detection & blur menggunakan **face-api.js**:

**Features:**
- Load TinyFaceDetector model dari `/public/models/`
- Deteksi wajah dengan threshold 0.3
- Multiple-pass blur (4x iterations) untuk efek kuat
- Auto-detect hardware threads

**Code Snippet:**
```javascript
useEffect(() => {
  const loadModels = async () => {
    await faceapi.nets.tinyFaceDetector.loadFromUri('/models');
    setModelReady(true);
  };
  loadModels();
}, []);

const handleProcess = async () => {
  const img = new Image();
  img.src = uploadedImage;
  
  const detections = await faceapi.detectAllFaces(
    img, 
    new faceapi.TinyFaceDetectorOptions({ 
      inputSize: 320,
      scoreThreshold: 0.3 
    })
  );

  // Blur each detected face
  detections.forEach((detection) => {
    const box = detection.box;
    // Extract → Blur 4x → Paste back
    for (let i = 0; i < 4; i++) {
      ctx.filter = `blur(${blurAmount}px)`;
      ctx.drawImage(tempCanvas, 0, 0);
    }
  });
};
```

**Functional Programming:**
- Pure transformations
- `.forEach()` dan `.map()` untuk iterasi
- Immutable image processing

---

### **Components Overview**

#### **RealtimeDetection.jsx**

Real-time webcam blur menggunakan backend mock detections:

- Polling `/api/detect` setiap 500ms
- Canvas rendering dengan blur filter
- HUD menampilkan jumlah wajah

#### **ImageUpload.jsx**

Drag-and-drop file upload dengan preview.

#### **ProcessSettings.jsx**

- **BlurSlider**: Adjust intensitas blur (5-50px)
- **Auto Thread Detection**: Otomatis gunakan `navigator.hardwareConcurrency`

---

## **API Endpoints**

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

Mock face detection untuk realtime demo.

**Response:**
```json
{
  "success": true,
  "num_faces": 2,
  "mock": true,
  "detections": [
    {
      "bbox": [120.5, 80.3, 250.7, 220.8],
      "confidence": 0.92,
      "class_id": 0
    },
    {
      "bbox": [340.2, 95.1, 470.8, 230.4],
      "confidence": 0.87,
      "class_id": 0
    }
  ]
}
```

**Fields:**
- `bbox`: `[x1, y1, x2, y2]` koordinat bounding box
- `confidence`: Skor confidence (0.0 - 1.0)
- `mock`: Flag untuk mock mode

---

## **Setup & Run**

### Prerequisites

1. **Rust** (rustup + cargo)
2. **Node.js** v18+
3. **npm** atau **pnpm**

### Installation

```powershell
# Clone repository
git clone https://github.com/RayhanMarcello/face-blur-detection.git
cd face-blur-detection

# Backend
cd backend
cargo build --release

# Frontend
cd ../frontend
npm install
```

### Running

**Terminal 1 - Backend:**
```powershell
cd backend
cargo run --release
# Server: http://127.0.0.1:3000
```

**Terminal 2 - Frontend:**
```powershell
cd frontend
npm run dev
# Frontend: http://localhost:5173
```

### Quick Test

```powershell
# Test health endpoint
Invoke-WebRequest -UseBasicParsing -Uri "http://127.0.0.1:3000/api/health"

# Test detection endpoint
Invoke-WebRequest -UseBasicParsing -Uri "http://127.0.0.1:3000/api/detect"
```

---

## **Features**

✅ **Upload Image Blur**
- Deteksi wajah menggunakan face-api.js (TinyFaceDetector)
- Multi-pass blur untuk hasil optimal
- Download hasil blur

✅ **Real-time Webcam Blur**
- Polling backend untuk mock detections
- Canvas overlay dengan blur filter
- Live face count display

✅ **Adjustable Settings**
- Blur intensity slider (5-50px)
- Auto hardware thread detection
- Processing stats (time, faces detected)

✅ **Modular Backend**
- Mock mode untuk development
- CORS enabled
- Type-safe error handling

---

## **Screenshots**

| Tampilan | Status |
|----------|--------|
| Health Check API | ✅ |
| Mock Detection Response | ✅ |
| Upload & Blur Interface | ✅ |
| Real-time Webcam Blur | ✅ |
| Processing Stats | ✅ |

---

## **Why Mock Backend?**

Backend mock digunakan karena:

1. **Kompleksitas Windows Setup**: OpenCV + ONNX Runtime memerlukan MSVC Build Tools, Windows SDK, CMake, vcpkg
2. **Development Speed**: Fokus pada arsitektur dan FP patterns tanpa dependency hell
3. **Client-side Accuracy**: face-api.js memberikan deteksi akurat di browser
4. **Easy Deployment**: Tidak perlu GPU atau native dependencies

### Migration ke Production

Untuk production dengan real inference:

```rust
// Ganti mock dengan real implementation
let model = YoloModel::load("yolo11n.onnx")?;  // ONNX Runtime
let camera = CameraSource::new(0, 640.0, 480.0)?;  // OpenCV
```

---

## **Conclusion**

Projek ini menunjukkan bahwa **Rust** dapat digunakan secara efektif untuk membangun layanan **privacy protection** yang memiliki kebutuhan:

✅ **Cepat & aman** dengan type-safe concurrency  
✅ Menerapkan paradigma **Functional Programming** secara konsisten  
✅ **Client-side processing** untuk privasi maksimal  
✅ **Modular architecture** yang mudah di-maintain  

### Functional Programming Benefits

- **Immutability** → Thread-safe tanpa explicit locks
- **Pure functions** → Predictable, testable, composable
- **Pattern matching** → Exhaustive error handling
- **Type safety** → Catch bugs at compile-time

### Future Enhancements

🔮 Ke depannya, fitur projek ini dapat dikembangkan menjadi:

- **Real backend inference** dengan ONNX Runtime + OpenCV
- **Batch video processing** untuk file upload
- **WebSocket streaming** untuk real-time collaboration
- **GPU acceleration** dengan CUDA support
- **Multi-model support** (face recognition, emotion detection)
- **Cloud deployment** dengan Docker/Kubernetes

---

## **Troubleshooting**

### Port Already in Use

```powershell
# Stop process on port 3000
Get-Process -Id (Get-NetTCPConnection -LocalPort 3000).OwningProcess | Stop-Process

# Or use different port
$env:PORT = "3001"
cargo run
```

### Frontend Can't Connect

- Pastikan backend running di port 3000
- Check CORS configuration
- Verify `VITE_API_BASE` environment variable

### face-api.js Model Loading Failed

```powershell
# Ensure models exist
ls frontend/public/models/

# Re-download if needed
cd frontend/public/models
# Download tiny_face_detector_model files
```

---

## **License**

MIT License - Feel free to use and modify! 🚀

---

## **Contact**

**Rayhan Marcello**  
GitHub: [@RayhanMarcello](https://github.com/RayhanMarcello)
