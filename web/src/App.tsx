import { createEffect } from "solid-js";
import init from "./wasm/rust-lox";

function App() {
  createEffect(() => {
    init().then(({ add }) => {
      console.log("from js: " + add(1, 1).toString());

      setTimeout(() => {
        console.log("from js: " + add(1, 1).toString());
      }, 1000);
    });
  });

  return (
    <>
      <h1>Wasm</h1>
    </>
  );
}

export default App;
