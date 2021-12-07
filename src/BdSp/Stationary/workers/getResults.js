import { expose } from 'comlink';
import { calculate_pokemon_bdsp_stationary } from '../../../../wasm/Cargo.toml';

expose(calculate_pokemon_bdsp_stationary);
