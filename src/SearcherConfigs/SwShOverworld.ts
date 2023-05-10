import type { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import {
  get_swsh_overworld_field_groups,
  get_swsh_overworld_result_columns,
  generate_swsh_overworld,
} from '~/../wasm/chatot/Cargo.toml';

export const SWSH_OVERWORLD_CONFIG: SearcherConfig = {
  getFieldGroups: get_swsh_overworld_field_groups,
  getResultColumns: get_swsh_overworld_result_columns,
  generateResults: generate_swsh_overworld,
};
