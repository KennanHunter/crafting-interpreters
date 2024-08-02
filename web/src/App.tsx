import { createEffect, createSignal } from "solid-js";
import init, { run } from "./wasm/rust-lox";

function App() {
  const [isLoaded, setIsLoaded] = createSignal(false);

  createEffect(() => {
    init().then(() => {
      setIsLoaded(true);
    });
  });

  const [code, setCode] = createSignal("");

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
          run(code());
        }}
      >
        Run
      </button>
    </div>
  );
}

export default App;
