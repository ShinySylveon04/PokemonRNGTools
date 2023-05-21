import React from 'react';
import { ColorMode } from './Components/ColorMode';
import { Pages } from './Pages';
import { MetaTags } from './Components/MetaTags';
import { WasmProvider } from './wasm/context';

export const App = () => {
  return (
    <React.Fragment>
      <MetaTags />
      <ColorMode>
        <WasmProvider>
          <Pages />
        </WasmProvider>
      </ColorMode>
    </React.Fragment>
  );
};
