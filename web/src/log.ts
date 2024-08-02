export const pushToLog = (input: string) => {
  const log = (document as any).useLogResults;

  log.setState({
    log: [...log.getState().log, input],
  });
};
