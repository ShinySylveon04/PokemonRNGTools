import { expose } from 'comlink';
import { get_bdsp_wild } from '../../../../wasm/Cargo.toml';

expose(get_bdsp_wild);
