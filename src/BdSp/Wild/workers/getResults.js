import { expose } from 'comlink';
import { get_wild } from '../../../../wasm/Cargo.toml';

expose(get_wild);
