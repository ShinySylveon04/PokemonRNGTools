import React from 'react';
import * as wasm from '../../wasm/chatot/pkg/chatot';
import type { WasmBindings } from './types';

type WasmState =
  | {
      isLoading: true;
      initializedWasm: null;
    }
  | {
      isLoading: false;
      initializedWasm: WasmBindings;
    };

const DEFAULT_STATE: WasmState = { isLoading: true, initializedWasm: null };

const WasmContext = React.createContext<WasmState>(DEFAULT_STATE);

export const WasmProvider = ({ children }: { children: React.ReactNode }) => {
  const [wasmState, setWasmState] = React.useState<WasmState>(DEFAULT_STATE);

  React.useEffect(() => {
    wasm
      .default()
      .then(() => setWasmState({ isLoading: false, initializedWasm: wasm }));
  }, []);

  return (
    <WasmContext.Provider value={wasmState}>{children}</WasmContext.Provider>
  );
};

export const useWasm = () => React.useContext(WasmContext);
