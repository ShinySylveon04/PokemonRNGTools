import { expose } from 'comlink';
import { get_bdsp_stationary } from '~/../wasm/chatot/Cargo.toml';

expose(get_bdsp_stationary);
