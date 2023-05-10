import type { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import {
  get_bdsp_underground_field_groups,
  get_bdsp_underground_result_columns,
  generate_bdsp_underground,
} from '~/../wasm/chatot/Cargo.toml';

export const BDSP_UNDERGROUND_CONFIG: SearcherConfig = {
  getFieldGroups: get_bdsp_underground_field_groups,
  getResultColumns: get_bdsp_underground_result_columns,
  generateResults: generate_bdsp_underground,
};
