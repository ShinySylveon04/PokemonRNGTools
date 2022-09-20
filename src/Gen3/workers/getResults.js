import { expose } from 'comlink';
import { get_gen3_wild } from '../../../wasm/Cargo.toml';

expose(get_gen3_wild);
