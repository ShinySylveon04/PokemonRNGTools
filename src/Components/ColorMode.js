import React from 'react';

import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

export const ColorModeContext = React.createContext({
  toggleColorMode: () => {},
});

export const ColorMode = ({ children }) => {
  const savedMode = JSON.parse(localStorage.getItem('ColorMode'));
  const [mode, setMode] = React.useState(savedMode ?? 'light');
  const theme = React.useMemo(() => createTheme({ palette: { mode } }), [mode]);

  React.useEffect(() => {
    localStorage.setItem('ColorMode', JSON.stringify(mode));
  }, [mode]);

  const colorMode = React.useMemo(
    () => ({
      toggleColorMode: () => {
        setMode(prevMode => (prevMode === 'light' ? 'dark' : 'light'));
      },
    }),
    [],
  );

  return (
    <ColorModeContext.Provider value={colorMode}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </ColorModeContext.Provider>
  );
};
