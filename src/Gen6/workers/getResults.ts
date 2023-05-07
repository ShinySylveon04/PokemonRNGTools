import { expose } from 'comlink';
import { get_transporter } from '~/../wasm/chatot/Cargo.toml';

expose(get_transporter);
