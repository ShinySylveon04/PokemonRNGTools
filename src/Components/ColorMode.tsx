import React from 'react';

import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { PaletteMode } from '@mui/material';

export const ColorModeContext = React.createContext({
  toggleColorMode: () => {},
});

export const ColorMode = ({ children }) => {
  const savedMode =
    localStorage.getItem('ColorMode') === 'light' ? 'light' : 'dark';
  const [mode, setMode] = React.useState<PaletteMode>(savedMode ?? 'light');
  const theme = React.useMemo(() => createTheme({ palette: { mode } }), [mode]);

  React.useEffect(() => {
    localStorage.setItem('ColorMode', mode);
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
