import "./App.css";
import { useEffect, useState } from "react";
import init, { run } from "./wasm/rust-lox";
import { useLogResults } from "./store";
import { create } from "zustand";
import { persist } from "zustand/middleware";

const useCodeState = create<{
  code: string;
  setCode: (newCode: string) => void;
}>()(
  persist(
    (set) => ({
      code: `
let test_variable = "hello";

print(test_variable + " world");

print(5 + 3 * 2);
`,
      setCode: (newCode) => {
        set({
          code: newCode,
        });
      },
    }),
    {
      name: "code-storage",
    }
  )
);

function App() {
  const { code, setCode } = useCodeState();
  const [isLoaded, setIsLoaded] = useState(false);

  const { log, clear } = useLogResults();

  useEffect(() => {
    init().then(() => {
      setIsLoaded(true);
    });
  });

  return (
    <div
      style={{
        display: "grid",
        gap: "1em",
        height: "100%",
        width: "90%",
        gridTemplateColumns: "1fr 6em 1fr",
        padding: "10px 5%",
      }}
    >
      <textarea
        style={{
          width: "100%",
          height: "100%",
        }}
        value={code}
        onInput={(e) => setCode(e.currentTarget.value)}
      />

      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "0.5em",
        }}
      >
        <button
          disabled={!isLoaded}
          onClick={() => {
            run(code);
          }}
        >
          Run
        </button>
        <button
          onClick={() => {
            clear();
          }}
        >
          Clear
        </button>
      </div>

      <pre
        style={{
          border: "1px solid black",
          borderRadius: "2px",
          width: "100%",
          height: "100%",
          margin: 0,
          padding: 0,
          whiteSpace: "pre-wrap",
          overflowWrap: "anywhere",
        }}
      >
        {log.join("\n")}
      </pre>
    </div>
  );
}

export default App;
