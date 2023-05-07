import { expose } from 'comlink';
import { get_bdsp_tid } from '~/../wasm/Cargo.toml';

expose(get_bdsp_tid);
