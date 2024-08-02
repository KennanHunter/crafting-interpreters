import { createEffect, createSignal, onMount } from "solid-js";
import init, { run } from "./wasm/rust-lox";
import { getLog, pushToLog } from "./log";

function App() {
  const [isLoaded, setIsLoaded] = createSignal(false);

  createEffect(() => {
    init().then(() => {
      setIsLoaded(true);
    });
  });

  const [code, setCode] = createSignal("");

  let logRef: HTMLPreElement | undefined;

  onMount(() => {
    document.addEventListener("storage", (_event) => {
      console.log("storage event listener triggered");

      if (!logRef) return;

      logRef.innerText = getLog();
    });
  });

  return (
    <div>
      <textarea
        value={code()}
        onInput={(e) => setCode(e.currentTarget.value)}
      />
      <br />
      <button
        disabled={!isLoaded()}
        onClick={() => {
          pushToLog("clicked");

          run(code());
        }}
      >
        Run
      </button>

      <pre
        style={{
          border: "2px solid black",
          width: "15em",
          height: "10em",
        }}
        ref={logRef}
      />
    </div>
  );
}

export default App;
