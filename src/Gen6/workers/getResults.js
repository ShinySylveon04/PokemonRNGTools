import { expose } from 'comlink';
import { get_transporter } from '../../../wasm/Cargo.toml';

expose(get_transporter);
