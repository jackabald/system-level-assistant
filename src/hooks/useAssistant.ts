import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type SystemContext = { app: string; clipboard: string };

export function useAssistant(pollMs = 5000) {
  const [ctx, setCtx] = useState<SystemContext | null>(null);
  const [reply, setReply] = useState("");
  const lastHash = useRef("");

  useEffect(() => {
    const tick = async () => {
      // get system context
      const context = await invoke<SystemContext>("get_context");
      setCtx(context);

      // only call AI if context changed
      const hash = JSON.stringify(context);
      if (hash !== lastHash.current && context.clipboard.trim().length > 0) {
        lastHash.current = hash;
        const aiText = await invoke<string>("get_ai_response", { context });
        setReply(aiText);
      }
    };

    tick();                          // initial fetch
    const id = setInterval(tick, pollMs);
    return () => clearInterval(id);  // cleanup
  }, [pollMs]);

  return { ctx, reply };
}
