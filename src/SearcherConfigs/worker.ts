import { expose } from 'comlink';
import init, {
  generate_bdsp_static,
  generate_tid,
  generate_bdsp_underground,
  generate_bdsp_wild,
  generate_gen3_wild,
  generate_swsh_overworld,
  generate_transporter,
} from '../../wasm/chatot/pkg/chatot';

export type ResultGenerator =
  | 'BdSpStatic'
  | 'BdSpTid'
  | 'BdSpUnderground'
  | 'BdSpWild'
  | 'Gen3Wild'
  | 'SwshOverworld'
  | 'Transporter';

// TS will error if we add a generator type and don't use it.
function exhaustive(generator: never) {
  throw new Error(`Got bad generator ${generator}`);
}

// Initialize a single wasm instance
const wasmInstance = init();

async function generateResults(generator: ResultGenerator, settings: unknown) {
  // Ensure the single instance has finished initializing
  await wasmInstance;
  switch (generator) {
    case 'BdSpStatic':
      return generate_bdsp_static(settings);
    case 'BdSpTid':
      return generate_tid(settings);
    case 'BdSpUnderground':
      return generate_bdsp_underground(settings);
    case 'BdSpWild':
      return generate_bdsp_wild(settings);
    case 'Gen3Wild':
      return generate_gen3_wild(settings);
    case 'SwshOverworld':
      return generate_swsh_overworld(settings);
    case 'Transporter':
      return generate_transporter(settings);
    default:
      return exhaustive(generator);
  }
}

expose(generateResults);
