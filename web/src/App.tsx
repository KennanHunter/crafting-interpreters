import { createEffect } from "solid-js";
import init from "./wasm/rust-lox";

function App() {
  createEffect(() => {
    init().then((initOutput) => {
      console.log("from js: " + initOutput.add(1, 1).toString());
    });
  });

  return (
    <>
      <h1>Wasm</h1>
    </>
  );
}

export default App;
