import { useState } from "react";

export const useImageProcessor = () => {
  const [uploadedImage, setUploadedImage] = useState(null);
  const [processedImage, setProcessedImage] = useState(null);
  const [isProcessing, setIsProcessing] = useState(false);

  const [blurIntensity, setBlurIntensity] = useState(20);
  const [threadCount, setThreadCount] = useState(4);

  const [stats, setStats] = useState({
    processingTime: 0,
    facesDetected: 0,
    threadsUsed: 0,
    status: "waiting",
  });

  const handleImageUpload = (file) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      setUploadedImage(e.target.result);
      setProcessedImage(null);
      resetStats();
    };
    reader.readAsDataURL(file);
  };

  const handleProcess = () => {
    if (!uploadedImage) return;

    setIsProcessing(true);
    updateStats("processing");

    setTimeout(() => {
      setProcessedImage(uploadedImage);

      setStats({
        processingTime: 2.5,
        facesDetected: 1,
        threadsUsed: threadCount,
        status: "completed",
      });

      setIsProcessing(false);
    }, 2500);
  };

  const handleReset = () => {
    setUploadedImage(null);
    setProcessedImage(null);
    resetStats();
  };

  const updateStats = (status) => {
    setStats((prev) => ({ ...prev, status }));
  };

  const resetStats = () => {
    setStats({
      processingTime: 0,
      facesDetected: 0,
      threadsUsed: 0,
      status: "waiting",
    });
  };

  return {
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
  };
};
