import { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import {
  get_bdsp_tid_field_groups,
  get_bdsp_tid_result_columns,
  generate_tid,
} from '~/../wasm/chatot/Cargo.toml';

export const BDSP_TID_CONFIG: SearcherConfig = {
  getFieldGroups: get_bdsp_tid_field_groups,
  getResultColumns: get_bdsp_tid_result_columns,
  generateResults: generate_tid,
};
