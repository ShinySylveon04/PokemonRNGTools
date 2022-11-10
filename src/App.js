import React from 'react';
export const ColorModeContext = React.createContext({
  toggleColorMode: () => {},
});

import { Pages } from './Pages';
import { MetaTags } from './Components/MetaTags';

import { ThemeProvider, createTheme } from '@mui/material/styles';

export const App = () => {
  const [mode, setMode] = React.useState('light');
  const colorMode = React.useMemo(
    () => ({
      toggleColorMode: () => {
        setMode(prevMode => (prevMode === 'light' ? 'dark' : 'light'));
      },
    }),
    [],
  );

  const theme = React.useMemo(
    () =>
      createTheme({
        palette: {
          mode,
        },
      }),
    [mode],
  );

  return (
    <React.Fragment>
      <MetaTags />
      <ColorModeContext.Provider value={colorMode}>
        <ThemeProvider theme={theme}>
          <Pages />
        </ThemeProvider>
      </ColorModeContext.Provider>
    </React.Fragment>
  );
};
