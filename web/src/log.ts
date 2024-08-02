export const pushToLog = (input: string) => {
  window.sessionStorage.setItem("log", getLog() + input);
};

export const getLog = (): string => {
  return window.sessionStorage.getItem("log") ?? "";
};

export const clearLog = () => {
  window.sessionStorage.removeItem("log");
};
