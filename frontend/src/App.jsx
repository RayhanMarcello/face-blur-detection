import "./style/tailwind.css";
import { useImageProcessor } from "./hooks/useImageProcessor";

import Header from "./components/layout/Header";
import Container from "./components/layout/Container";

import ImageUpload from "./components/upload/ImageUpload";
import ImagePreview from "./components/upload/ImagePreview";
import RealtimeDetection from "./components/upload/RealtimeDetection";
import DownloadResult from "./components/upload/DownloadResult";

import ProcessSettings from "./components/processing/ProcessSettings";
import ProcessStats from "./components/results/ProcessStats";

import FeatureList from "./components/features/FeatureList";
import Button from "./components/ui/Button";

function App() {
  const {
    uploadedImage,
    processedImage,
    isProcessing,

    blurIntensity,
    threadCount,
    stats,

    handleImageUpload,
    handleProcess,
    handleReset,

    setBlurIntensity,
    setThreadCount,
  } = useImageProcessor();

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
      <Header />

      <Container>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          
          <section className="lg:col-span-2">
            <div className="bg-white rounded-2xl shadow-lg p-4 sm:p-6 md:p-8">

              {!uploadedImage ? (
                <ImageUpload onUpload={handleImageUpload} />
              ) : (
                <ImagePreview
                  originalImage={uploadedImage}
                  processedImage={processedImage}
                  isProcessing={isProcessing}
                />
              )}

              {uploadedImage && (
                <div className="mt-6 flex flex-col sm:flex-row gap-4">
                  <Button
                    fullWidth
                    variant="primary"
                    disabled={isProcessing}
                    onClick={handleProcess}
                  >
                    {isProcessing ? "⏳ Memproses..." : "▶ Mulai Proses"}
                  </Button>

                  <Button variant="secondary" onClick={handleReset}>
                    🔄 Reset
                  </Button>
                </div>
              )}

              {uploadedImage && (
                <div className="mt-10 space-y-8">
                  <RealtimeDetection onFrame={() => {}} />
                  <DownloadResult imageUrl={processedImage} />
                </div>
              )}

              <FeatureList />
            </div>
          </section>

          <aside className="space-y-6">
            <ProcessSettings
              blurIntensity={blurIntensity}
              onBlurChange={setBlurIntensity}
              threadCount={threadCount}
              onThreadChange={setThreadCount}
            />

            <ProcessStats stats={stats} />
          </aside>

        </div>
      </Container>
    </div>
  );
}

export default App;
