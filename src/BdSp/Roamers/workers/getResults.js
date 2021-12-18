import { expose } from 'comlink';
import { calculate_pokemon_bdsp_roamer } from '../../../../wasm/Cargo.toml';

expose(calculate_pokemon_bdsp_roamer);
