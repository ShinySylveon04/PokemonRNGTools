import { expose } from 'comlink';
import * as wasm from '../../wasm/chatot/pkg/chatot';
import type { GeneratorFunctionName } from 'wasm/types';

// Initialize a single wasm instance
const wasmInstance = wasm.default();

async function generateResults(
  generator: GeneratorFunctionName,
  settings: unknown,
) {
  // Ensure the single instance has finished initializing
  await wasmInstance;
  return wasm[generator](settings);
}

expose(generateResults);
