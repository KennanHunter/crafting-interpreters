import { createEffect } from "solid-js";
import init from "./wasm/rust-lox";

function App() {
  createEffect(() => {
    init().then(({ run }) => {
      (run as unknown as (input: string) => void)("print(1 + 1);");
    });
  });

  return (
    <>
      <h1>Wasm</h1>
    </>
  );
}

export default App;
