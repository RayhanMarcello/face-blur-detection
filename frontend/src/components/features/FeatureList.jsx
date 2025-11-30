import FeatureCard from "./FeatureCard";

const FeatureList = () => {
  const features = [
    {
      icon: "⚡",
      title: "Pemrosesan Cepat",
      description: "Multi-threading untuk performa optimal",
      color: "blue",
    },
    {
      icon: "🎯",
      title: "Deteksi Akurat",
      description: "AI detection untuk hasil maksimal",
      color: "purple",
    },
    {
      icon: "🎨",
      title: "Kualitas Terjaga",
      description: "Tidak menurunkan kualitas gambar",
      color: "green",
    },
  ];

  return (
    <div className="mt-8 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
      {features.map((f, i) => (
        <FeatureCard key={i} {...f} />
      ))}
    </div>
  );
};

export default FeatureList;
