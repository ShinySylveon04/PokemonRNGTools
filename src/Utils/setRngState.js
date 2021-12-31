export const setRngStateFromClipboard = (event, setState) => {
  const text = event.clipboardData.getData('Text').split('\n');

  if (text.length === 4) {
    setState(state => ({
      ...state,
      state0: text[0],
      state1: text[1],
      state2: text[2],
      state3: text[3],
    }));
  }
};
