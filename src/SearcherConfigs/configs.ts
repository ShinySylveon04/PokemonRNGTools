import { wrap } from 'comlink';
import type { ResultGenerator } from './worker';
import type { SearcherConfig } from '../Layouts/ConfiguableSearcher';
import {
  get_bdsp_static_field_groups,
  get_bdsp_static_result_columns,
  get_bdsp_tid_field_groups,
  get_bdsp_tid_result_columns,
  get_bdsp_underground_field_groups,
  get_bdsp_underground_result_columns,
  get_bdsp_wild_field_groups,
  get_bdsp_wild_result_columns,
  get_gen3_wild_field_groups,
  get_gen3_wild_result_columns,
  get_swsh_overworld_field_groups,
  get_swsh_overworld_result_columns,
  get_transporter_field_groups,
  get_transporter_result_columns,
} from '~/../wasm/chatot/Cargo.toml';

const generateResults = wrap<(name: ResultGenerator, settings: unknown) => any>(
  new Worker('./worker.ts'),
);

export const BDSP_STATIC_CONFIG: SearcherConfig = {
  getFieldGroups: get_bdsp_static_field_groups,
  getResultColumns: get_bdsp_static_result_columns,
  generateResults: settings => generateResults('BdSpStatic', settings),
};

export const BDSP_TID_CONFIG: SearcherConfig = {
  getFieldGroups: get_bdsp_tid_field_groups,
  getResultColumns: get_bdsp_tid_result_columns,
  generateResults: settings => generateResults('BdSpTid', settings),
};

export const BDSP_UNDERGROUND_CONFIG: SearcherConfig = {
  getFieldGroups: get_bdsp_underground_field_groups,
  getResultColumns: get_bdsp_underground_result_columns,
  generateResults: settings => generateResults('BdSpUnderground', settings),
};

export const BDSP_WILD_CONFIG: SearcherConfig = {
  getFieldGroups: get_bdsp_wild_field_groups,
  getResultColumns: get_bdsp_wild_result_columns,
  generateResults: settings => generateResults('BdSpWild', settings),
};

export const GEN3_WILD_CONFIG: SearcherConfig = {
  getFieldGroups: get_gen3_wild_field_groups,
  getResultColumns: get_gen3_wild_result_columns,
  generateResults: settings => generateResults('Gen3Wild', settings),
};

export const SWSH_OVERWORLD_CONFIG: SearcherConfig = {
  getFieldGroups: get_swsh_overworld_field_groups,
  getResultColumns: get_swsh_overworld_result_columns,
  generateResults: settings => generateResults('SwshOverworld', settings),
};

export const TRANSPORTER_CONFIG: SearcherConfig = {
  getFieldGroups: get_transporter_field_groups,
  getResultColumns: get_transporter_result_columns,
  generateResults: settings => generateResults('Transporter', settings),
};
