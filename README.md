# Face Blur Detection – Privacy Protection System
*A Functional Programming Approach with Rust & React*

**Author:**  
Rayhan Marcello Ananda Purnomo | Maulida Rahmayanti | Muhammad Rakha Randhika | Faqih Chairul Anam | Amisha Nabila | Nurhafid Sudarianto
---

## Abstract
Face Blur Detection adalah aplikasi perlindungan privasi yang secara otomatis mendeteksi dan mengaburkan wajah pada gambar, membantu menjaga data visual tetap aman. Backend dikembangkan menggunakan Rust dengan Axum dan Tokio, menerapkan prinsip functional programming agar proses deteksi lebih aman, efisien, dan bebas error. Di sisi frontend, React (Vite) bersama face-api.js digunakan untuk mendeteksi wajah secara real-time dengan akurat di browser. Proyek ini menunjukkan bagaimana Rust dan functional programming bisa bekerja sama menciptakan layanan deteksi wajah yang cepat, aman, dan siap digunakan dalam berbagai aplikasi yang membutuhkan perlindungan privasi.
---

## Introduction
Di zaman digital seperti sekarang, berbagi dan menyimpan gambar menjadi sangat mudah, namun hal ini juga menimbulkan tantangan terkait privasi. Wajah seseorang bisa tersebar atau digunakan tanpa izin mereka. Face Blur Detection dibuat untuk mengatasi masalah ini dengan mendeteksi dan mengaburkan wajah dalam gambar secara otomatis, sehingga privasi pengguna tetap terlindungi.
Aplikasi ini dirancang untuk menyelesaikan beberapa permasalahan utama dalam perlindungan privasi visual:
1. Otomatisasi blur wajah untuk konten sensitif  
2. Pemrosesan real-time dengan latensi rendah  
3. Kebutuhan sistem modern yang aman, efisien, dan scalable  

### Mengapa menggunakan Rust?
| Alasan                  | Penjelasan                                                                                                                                                                  |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Memory Safety**       | Rust memastikan keamanan memori melalui ownership dan borrowing system. Zero-cost abstractions memungkinkan performa tinggi tanpa overhead garbage collector.               |
| **High Concurrency**    | Dengan **Tokio async runtime**, Rust dapat menangani banyak request secara bersamaan dengan efisien, membuat aplikasi scalable dan responsif.                               |
| **Functional Friendly** | Rust mendukung paradigma pemrograman fungsional, seperti pure functions, immutability, dan higher-order functions, sehingga kode lebih modular, mudah diuji, dan minim bug. |
| **Type Safety**         | Sistem tipe yang kuat di Rust meminimalkan kesalahan saat runtime dengan mendeteksi error lebih awal di compile-time, membuat aplikasi lebih stabil.                        |
| **Performance**         | Rust menghasilkan executable yang sangat cepat karena optimisasi compiler dan manajemen memori yang efisien, cocok untuk aplikasi real-time seperti face detection.         |
| **Cross-platform**      | Rust dan ekosistemnya mendukung pengembangan lintas platform, memudahkan integrasi dengan frontend berbasis web atau desktop.                                               |
| **Error Handling**      | Rust menggunakan **Result** dan **Option** types untuk error handling yang aman dan eksplisit, mengurangi kemungkinan crash tak terduga.                                    |

### Integrasi Functional Programming
Penerapan paradigma functional programming membantu membuat kode menjadi lebih rapi, modular, dan mudah diuji. Pendekatan ini memudahkan pengelolaan pipeline deteksi wajah yang kompleks, sekaligus menjaga keamanan dan meminimalkan potensi kesalahan.

### Keunikan Solusi
- Real-time di browser: Deteksi wajah dilakukan secara langsung di browser dengan akurasi tinggi menggunakan face-api.js.
- Backend efisien dan aman: Rust memastikan proses server berjalan cepat dan stabil, sekaligus aman dari kesalahan memori.
- Layanan cepat dan andal: Kombinasi frontend dan backend menghasilkan aplikasi yang responsif, aman, dan siap digunakan untuk berbagai kebutuhan perlindungan privasi.
---

## Background & Concepts
Untuk memahami Face Blur Detection, beberapa konsep kunci perlu dipahami:

1. Face Detection
Face detection adalah proses mengidentifikasi lokasi wajah manusia dalam gambar atau video. Ini merupakan langkah awal sebelum melakukan pengaburan atau manipulasi data visual untuk menjaga privasi. Teknologi ini menggunakan algoritma machine learning untuk mendeteksi fitur wajah dengan akurat.

2. Privacy Protection
Perlindungan privasi digital semakin penting di era media sosial dan cloud storage. Dengan menyamarkan wajah dalam gambar, aplikasi ini membantu mencegah penyalahgunaan data pribadi dan kebocoran identitas.

3. Rust Programming Language
Rust adalah bahasa pemrograman sistem yang menekankan performa tinggi, keamanan memori, dan concurrency. Rust memungkinkan pengembangan backend yang cepat dan stabil, cocok untuk aplikasi real-time seperti Face Blur Detection.

4. Functional Programming
Functional programming adalah paradigma yang menekankan immutability, pure functions, dan modularitas. Pendekatan ini membuat kode lebih mudah diuji, aman, dan minim error, terutama dalam pipeline deteksi wajah yang kompleks.

5. Frontend Integration (React + face-api.js)
Frontend menggunakan React (Vite) untuk membangun antarmuka yang responsif, sementara face-api.js memungkinkan deteksi wajah secara real-time di browser. Kombinasi ini memastikan pengalaman pengguna yang lancar sekaligus akurat.

6. Asynchronous Programming (Tokio)
Rust menggunakan Tokio async runtime untuk menjalankan banyak proses secara bersamaan tanpa menghambat performa server. Ini penting agar backend mampu menangani banyak request real-time dengan efisien.

### Technology Stack
| Komponen       | Teknologi                      | Fungsi / Keterangan                                                                            |
| -------------- | ------------------------------ | ---------------------------------------------------------------------------------------------- |
| Backend        | Rust + Axum                    | Backend cepat, aman, dan terstruktur; Axum memudahkan pengembangan API.                        |
| Frontend       | React (Vite) + Tailwind CSS    | React untuk antarmuka interaktif, Vite untuk build cepat, Tailwind CSS untuk desain responsif. |
| Runtime Async  | Tokio                          | Menangani banyak proses bersamaan agar backend tetap responsif.                                |
| Face Detection | face-api.js (TinyFaceDetector) | Deteksi wajah real-time di browser dengan model ringan dan akurat.                             |
| Serialization  | Serde                          | Memudahkan serialisasi dan deserialisasi data antara backend dan frontend.                     |
| CORS           | tower-http                     | Mengatur kebijakan CORS agar komunikasi frontend-backend aman.                                 |

### Konsep Functional Programming dalam Sistem
| Konsep FP              | Implementasi dalam Proyek                                     |
|------------------------|---------------------------------------------------------------|
| Pure Functions         | Handler detection tidak memodifikasi state global             |
| Immutability           | `AppState` dibungkus dalam `Arc` untuk thread-safe sharing    |
| Pattern Matching       | Error handling dengan `Result<T, AppError>` dan `match`       |
| Higher-Order Functions | Penggunaan `.map()` dan `.filter()` untuk transformasi data   |
| Composition            | Pipeline: Upload → Detect → Blur → Download                   |

Dengan pendekatan ini, aplikasi dapat menangani banyak request secara serentak tanpa bottleneck yang signifikan.
---

## Source Code Overview

### Struktur Folder

```text
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
---
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
    println!(" Starting Face Blur Detection Server (Mock Mode)...");

    let model = YoloModel::new_mock();
    let camera = CameraSource::new_mock();
    let state = AppState::new(camera, model);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = api_router(Arc::new(state)).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!(" Server running on http://127.0.0.1:3000");
    
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

async fn camera() -> Json<Value> {
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
---
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

Upload Image and realtime camera blur menggunakan backend mock detections:

- Polling `/api/detect` setiap 500ms
- Canvas rendering dengan blur filter
- HUD menampilkan jumlah wajah

#### **ImageUpload.jsx**

Drag-and-drop file upload dengan preview.

#### **ProcessSettings.jsx**

- **BlurSlider**: Adjust intensitas blur (5-50px)
- **Auto Thread Detection**: Otomatis gunakan `navigator.hardwareConcurrency`

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
---
### Prerequisites

1. **Rust** (rustup + cargo)
2. **Node.js** v18+
3. **npm** atau **pnpm**

### Installation
---
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
---
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

## **Features**
---
**Upload Image Blur**
- Deteksi wajah menggunakan face-api.js (TinyFaceDetector)
- Multi-pass blur untuk hasil optimal
- Download hasil blur

**Real-time Webcam Blur**
- Polling backend untuk mock detections
- Canvas overlay dengan blur filter
- Live face count display

**Adjustable Settings**
- Blur intensity slider (5-50px)
- Auto hardware thread detection
- Processing stats (time, faces detected)

**Modular Backend**
- Mock mode untuk development
- CORS enabled
- Type-safe error handling


Untuk production dengan real inference:
---
```rust
// Ganti mock dengan real implementation
let model = YoloModel::load("yolo11n.onnx")?;  // ONNX Runtime
let camera = CameraSource::new(0, 640.0, 480.0)?;  // OpenCV
```
---

## **ScreenShot**
![WhatsApp Image 2025-12-01 at 04 29 44_3aeb85fd](https://github.com/user-attachments/assets/03b4edc3-1c50-4c82-95b3-52597529d601)
![WhatsApp Image 2025-12-01 at 04 29 24_e1ce408e](https://github.com/user-attachments/assets/291524d6-b093-4559-bb7a-16def4049b65)
---

## **Conclusion**
Face Blur Detection memberikan cara yang praktis untuk menjaga privasi digital dengan secara otomatis mendeteksi dan mengaburkan wajah dalam gambar. Backend yang dibangun menggunakan Rust membuat aplikasi berjalan cepat, aman dari masalah memori, dan mampu menangani banyak proses sekaligus. Frontend React (Vite) dan face-api.js memastikan pengalaman pengguna tetap lancar dan responsif.

Pendekatan functional programming membuat kode lebih rapi, modular, dan mudah diuji. Pipeline sistem yang jelas - Upload, Detect, Blur, Download - menjamin alur kerja tetap efisien dan mudah dipelihara. Kombinasi teknologi ini membuktikan bahwa menjaga privasi tidak perlu mengorbankan performa atau kenyamanan pengguna. Face Blur Detection menjadi solusi yang aman, cepat, dan siap digunakan dalam berbagai situasi.

### Future Enhancements
🔮 Ke depannya, fitur projek ini dapat dikembangkan menjadi:
- **Real backend inference** dengan ONNX Runtime + OpenCV
- **Batch video processing** untuk file upload
- **WebSocket streaming** untuk real-time collaboration
- **GPU acceleration** dengan CUDA support
- **Multi-model support** (face recognition, emotion detection)
- **Cloud deployment** dengan Docker/Kubernetes
---
