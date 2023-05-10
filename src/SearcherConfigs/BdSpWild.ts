import type { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import {
  get_bdsp_wild_field_groups,
  get_bdsp_wild_result_columns,
  generate_bdsp_wild,
} from '~/../wasm/chatot/Cargo.toml';

export const BDSP_WILD_CONFIG: SearcherConfig = {
  getFieldGroups: get_bdsp_wild_field_groups,
  getResultColumns: get_bdsp_wild_result_columns,
  generateResults: generate_bdsp_wild,
};
