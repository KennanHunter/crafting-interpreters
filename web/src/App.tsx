import { useEffect, useState } from "react";
import init, { run } from "./wasm/rust-lox";
import { useLogResults } from "./store";

function App() {
  const [code, setCode] = useState("");
  const [isLoaded, setIsLoaded] = useState(false);

  const { log, clear } = useLogResults();

  useEffect(() => {
    init().then(() => {
      setIsLoaded(true);
    });
  });

  return (
    <div>
      <textarea value={code} onInput={(e) => setCode(e.currentTarget.value)} />
      <br />
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

      <pre
        style={{
          border: "2px solid black",
          width: "15em",
          height: "10em",
        }}
      >
        {log.join("\n")}
      </pre>
    </div>
  );
}

export default App;
