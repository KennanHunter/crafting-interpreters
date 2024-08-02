import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface LogResult {
  log: string[];
  push: (line: string) => void;
  clear: () => void;
}

export const useLogResults = create<LogResult>()(
  persist(
    (set, get) => ({
      log: [],
      push: (line: string) => {
        set({ log: [...get().log, line] });
      },
      clear: () => {
        set({ log: [] });
      },
    }),
    {
      name: "log-results",
      storage: createJSONStorage(() => sessionStorage),
    }
  )
);

(document as any).useLogResults = useLogResults;
