import { expose } from 'comlink';
import { get_bdsp_stationary } from '../../../../wasm/Cargo.toml';

expose(get_bdsp_stationary);
