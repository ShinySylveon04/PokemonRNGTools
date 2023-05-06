import { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import {
  get_transporter_field_groups,
  get_transporter_result_columns,
  generate_transporter,
} from '~/../wasm/Cargo.toml';

export const TRANSPORTER_CONFIG: SearcherConfig = {
  getFieldGroups: get_transporter_field_groups,
  getResultColumns: get_transporter_result_columns,
  generateResults: generate_transporter,
};
