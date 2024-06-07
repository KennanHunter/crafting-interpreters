import init from "./assets/rust-lox.wasm?init";
import { ModuleExports } from "./wasm-typings";

function App() {
  init().then((mod) => {
    let { add } = mod.exports as ModuleExports;
    let res = add(5, 5);
    console.log(res);
  });

  return (
    <>
      <h1>Wasm</h1>
    </>
  );
}

export default App;
