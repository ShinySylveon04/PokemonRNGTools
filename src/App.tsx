import React from 'react';
import CircularProgress from '@mui/material/CircularProgress';
import Box from '@mui/material/Box';
import { ColorMode } from './Components/ColorMode';
import { Pages } from './Pages';
import { MetaTags } from './Components/MetaTags';
import init from '../wasm/chatot/pkg/chatot';

const useWasmInit = () => {
  const [result, setResult] = React.useState({ isLoading: true, error: null });

  React.useEffect(() => {
    init()
      .then(() => setResult({ isLoading: false, error: null }))
      .catch(error => setResult({ isLoading: false, error }));
  }, []);

  return result;
};

export const App = () => {
  const { isLoading } = useWasmInit();

  return (
    <React.Fragment>
      <MetaTags />
      <ColorMode>
        {isLoading ? (
          <Box
            display="flex"
            justifyContent="center"
            alignItems="center"
            width="100vw"
            height="100vh"
          >
            <CircularProgress />
          </Box>
        ) : (
          <Pages />
        )}
      </ColorMode>
    </React.Fragment>
  );
};
