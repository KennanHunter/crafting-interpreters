import {
  IconBrandGithub,
  IconPlayerPlay,
  IconRefresh,
} from "@tabler/icons-react";
import { useEffect, useState } from "react";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import "./App.css";
import { useLogResults } from "./store";
import init, { run } from "./wasm/rust-lox";

const useCodeState = create<{
  code: string;
  setCode: (newCode: string) => void;
  resetCode: () => void;
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
      resetCode: () => {
        set({
          code: useCodeState.getInitialState().code,
        });
      },
    }),
    {
      name: "code-storage",
    }
  )
);

function App() {
  const { code, setCode, resetCode } = useCodeState();
  const [isLoaded, setIsLoaded] = useState(false);

  const { log, clear } = useLogResults();

  useEffect(() => {
    init().then(() => {
      setIsLoaded(true);
    });
  });

  return (
    <div
      className="grid h-full"
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
        <div
          className=" grid border border-black rounded-sm whitespace-pre-wrap "
          style={{
            gridTemplateColumns: "1fr 8rem",
            gridTemplateRows: "1fr 8rem",
          }}
        >
          <textarea
            className="resize-none  w-full h-full bg-slate-800 p-2 overflow-y-scroll"
            value={code}
            onInput={(e) => setCode(e.currentTarget.value)}
            style={{
              gridColumn: "1/3",
              gridRow: "1/3",
            }}
          />
          <button
            style={{
              gridRowStart: 2,
              gridColumnStart: 2,
            }}
            className="m-auto border border-black shadow-lg p-4 rounded-md grid place-items-center bg-slate-900"
            onClick={() => {
              resetCode();
            }}
          >
            <IconRefresh />
          </button>
        </div>
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
        <div className="resize-none h-full bg-slate-800 border border-black rounded-sm whitespace-pre-wrap p-2 overflow-y-scroll">
          {log.join("\n")}
        </div>
      </div>
    </div>
  );
}

export default App;
