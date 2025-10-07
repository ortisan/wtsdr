export default function AuthLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen flex items-center justify-center p-6">
      <div className="w-full max-w-md bg-white/70 dark:bg-black/20 backdrop-blur rounded-lg shadow p-6">
        {children}
      </div>
    </div>
  );
}
