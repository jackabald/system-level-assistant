import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type SystemContext = {
  app: string;
  clipboard: string;
};

function App() {
  const [context, setContext] = useState<SystemContext | null>(null);

  useEffect(() => {
    const fetchContext = async () => {
      const result = await invoke<SystemContext>("get_context");
      setContext(result);
    };

    fetchContext();
    // Optional: poll every few seconds
    const interval = setInterval(fetchContext, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <main className="container">
      <h1 className="text-xl font-bold">System Context</h1>

      {context ? (
        <div className="mt-4">
          <p><strong>Active App:</strong> {context.app}</p>
          <p><strong>Clipboard:</strong> {context.clipboard}</p>
        </div>
      ) : (
        <p>Loading context...</p>
      )}
    </main>
  );
}

export default App;
