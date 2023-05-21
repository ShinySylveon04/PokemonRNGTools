import * as tst from 'ts-toolbelt';
import * as wasm from '../../wasm/chatot/pkg/chatot';

export type WasmBindings = typeof wasm;
export type NonFunctionBindings = tst.O.Filter<
  WasmBindings,
  tst.F.Function,
  'extends->'
>;
export type FunctionBindings = tst.O.Diff<WasmBindings, NonFunctionBindings>;

export type GeneratorFunctionName = tst.U.Filter<
  `generate_${string}`,
  keyof FunctionBindings,
  'extends->'
>;
export type GetFieldGroupFunctionName = tst.U.Filter<
  `${string}_field_groups`,
  keyof FunctionBindings,
  'extends->'
>;
export type GetResultColumnFunctionName = tst.U.Filter<
  `${string}_result_columns`,
  keyof FunctionBindings,
  'extends->'
>;
