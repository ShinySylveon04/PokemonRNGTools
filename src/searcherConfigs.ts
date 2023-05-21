import { wrap } from 'comlink';
import type { GeneratorFunctionName } from './wasm/types';
import type { SearcherConfig } from './Layouts/ConfiguableSearcher';

export const generateResults = wrap<
  (name: GeneratorFunctionName, settings: unknown) => any
>(new Worker(new URL('./wasm/worker.ts', import.meta.url), { type: 'module' }));

export const BDSP_STATIC_CONFIG: SearcherConfig = {
  getFieldGroups: 'get_bdsp_static_field_groups',
  getResultColumns: 'get_bdsp_static_result_columns',
  generateResults: settings =>
    generateResults('generate_bdsp_static', settings),
};

export const BDSP_TID_CONFIG: SearcherConfig = {
  getFieldGroups: 'get_bdsp_tid_field_groups',
  getResultColumns: 'get_bdsp_tid_result_columns',
  generateResults: settings => generateResults('generate_tid', settings),
};

export const BDSP_UNDERGROUND_CONFIG: SearcherConfig = {
  getFieldGroups: 'get_bdsp_underground_field_groups',
  getResultColumns: 'get_bdsp_underground_result_columns',
  generateResults: settings =>
    generateResults('generate_bdsp_underground', settings),
};

export const BDSP_WILD_CONFIG: SearcherConfig = {
  getFieldGroups: 'get_bdsp_wild_field_groups',
  getResultColumns: 'get_bdsp_wild_result_columns',
  generateResults: settings => generateResults('generate_bdsp_wild', settings),
};

export const GEN3_WILD_CONFIG: SearcherConfig = {
  getFieldGroups: 'get_gen3_wild_field_groups',
  getResultColumns: 'get_gen3_wild_result_columns',
  generateResults: settings => generateResults('generate_gen3_wild', settings),
};

export const SWSH_OVERWORLD_CONFIG: SearcherConfig = {
  getFieldGroups: 'get_swsh_overworld_field_groups',
  getResultColumns: 'get_swsh_overworld_result_columns',
  generateResults: settings =>
    generateResults('generate_swsh_overworld', settings),
};

export const TRANSPORTER_CONFIG: SearcherConfig = {
  getFieldGroups: 'get_transporter_field_groups',
  getResultColumns: 'get_transporter_result_columns',
  generateResults: settings =>
    generateResults('generate_transporter', settings),
};
