import type { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import {
  get_bdsp_static_field_groups,
  get_bdsp_static_result_columns,
  generate_bdsp_static,
} from '~/../wasm/chatot/Cargo.toml';

export const BDSP_STATIC_CONFIG: SearcherConfig = {
  getFieldGroups: get_bdsp_static_field_groups,
  getResultColumns: get_bdsp_static_result_columns,
  generateResults: generate_bdsp_static,
};
