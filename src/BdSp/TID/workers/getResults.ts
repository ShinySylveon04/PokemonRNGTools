import { expose } from 'comlink';
import { get_bdsp_tid } from '~/../wasm/chatot/Cargo.toml';

expose(get_bdsp_tid);
