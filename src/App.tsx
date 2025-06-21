import { useAssistant } from "./hooks/useAssistant";
import "./App.css";

export default function App() {
  const { ctx, reply } = useAssistant(5000); // poll every 5 s

  return (
    <main className="container">
      <h1 className="text-xl font-bold mb-4">System-Level Assistant</h1>

      {ctx && (
        <div className="mb-6">
          <p><strong>Active App:</strong> {ctx.app}</p>
          <p><strong>Clipboard:</strong> {ctx.clipboard}</p>
        </div>
      )}

      {reply && (
        <div className="p-4 border rounded shadow">
          <strong>Assistant&nbsp;Says:</strong>
          <p className="mt-2">{reply}</p>
        </div>
      )}
    </main>
  );
}
