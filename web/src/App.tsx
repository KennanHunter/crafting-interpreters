import { ModuleExports } from "./wasm-typings";
import init from "./assets/rust-lox.wasm?init";

function App() {
  init()
    .catch((err) => {
      console.log(
        "Failed to import wasm module, " + JSON.stringify(err, null, "\t")
      );
    })
    .then((mod) => {
      if (!mod) return;

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
