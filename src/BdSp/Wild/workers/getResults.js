import { expose } from 'comlink';
import { calculate_pokemon_bdsp } from '../../../../wasm/Cargo.toml';

expose(calculate_pokemon_bdsp);
