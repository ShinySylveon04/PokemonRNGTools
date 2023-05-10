import type { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import {
  get_gen3_wild_field_groups,
  get_gen3_wild_result_columns,
  generate_gen3_wild,
} from '~/../wasm/chatot/Cargo.toml';

export const GEN3_WILD_CONFIG: SearcherConfig = {
  getFieldGroups: get_gen3_wild_field_groups,
  getResultColumns: get_gen3_wild_result_columns,
  generateResults: generate_gen3_wild,
};
