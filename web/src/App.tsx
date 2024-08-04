import "./App.css";
import { useEffect, useState } from "react";
import init, { run } from "./wasm/rust-lox";
import { useLogResults } from "./store";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import { IconBrandGithub, IconPlayerPlay } from "@tabler/icons-react";

const useCodeState = create<{
  code: string;
  setCode: (newCode: string) => void;
}>()(
  persist(
    (set) => ({
      code: `\
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
      className="grid h-full text-white bg-slate-900"
      style={{
        gridTemplateRows: "4em 1fr",
      }}
    >
      <div className="bg-slate-800 flex w-full justify-start align-middle items-center px-4 rounded-b-md">
        <h1>Kennan's Wasm Lox Interpreter</h1>
        <a
          className="ml-auto"
          href="https://github.com/kennanhunter/crafting-interpreters"
        >
          <button className="border border-black aspect-square  rounded-sm">
            <IconBrandGithub width={50} />
          </button>
        </a>
      </div>
      <div
        className="grid gap-4 h-full w-full p-6"
        style={{
          gridTemplateColumns: "1fr 4em 1fr",
        }}
      >
        <textarea
          className="resize-none h-full bg-slate-800 border border-black rounded-sm whitespace-pre-wrap overflow p-2"
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
            className="border bg-slate-800 border-black rounded-lg aspect-square grid place-content-center"
            onClick={() => {
              run(code);
            }}
          >
            <IconPlayerPlay height={"100%"} />
          </button>
          <button
            className="border bg-slate-800 border-black rounded-lg aspect-square grid place-content-center"
            onClick={() => {
              clear();
            }}
          >
            Clear
          </button>
        </div>
        <pre
          className="border bg-slate-800 border-black rounded-sm whitespace-pre-wrap overflow p-2"
          style={{
            overflowWrap: "anywhere",
          }}
        >
          {log.join("\n")}
        </pre>
      </div>
    </div>
  );
}

export default App;
