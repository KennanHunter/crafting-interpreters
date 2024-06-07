import init from "./assets/rust-lox.wasm?init";
import { ModuleExports } from "./wasm-typings";

function App() {
  init().then((mod) => {
    let { run } = mod.exports as ModuleExports;
    run("print 1 + 1;");
  });

  return (
    <>
      <h1>Wasm</h1>
    </>
  );
}

export default App;
