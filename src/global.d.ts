// Temporary so we don't have to port all rust types over right now
type AnyPromiseFunction = (...props: any[]) => Promise<any>;
type AnyFunction = (...props: any[]) => any;

declare module '~/../wasm/chatot/Cargo.toml' {
  const get_bdsp_stationary: AnyPromiseFunction;
  const get_bdsp_tid: AnyPromiseFunction;
  const calculate_pokemon_bdsp_underground: AnyPromiseFunction;
  const get_bdsp_wild: AnyPromiseFunction;
  const get_gen3_wild: AnyPromiseFunction;
  const get_transporter: AnyPromiseFunction;
  const calculate_pokemon: AnyFunction;
  const get_transporter_field_groups: AnyFunction;
  const get_transporter_result_columns: AnyFunction;
  const generate_transporter: AnyFunction;
  const get_gen3_wild_field_groups: AnyFunction;
  const get_gen3_wild_result_columns: AnyFunction;
  const generate_gen3_wild: AnyFunction;
  const get_bdsp_wild_field_groups: AnyFunction;
  const get_bdsp_wild_result_columns: AnyFunction;
  const generate_bdsp_wild: AnyFunction;
  const get_bdsp_tid_field_groups: AnyFunction;
  const get_bdsp_tid_result_columns: AnyFunction;
  const generate_tid: AnyFunction;
  const get_bdsp_static_field_groups: AnyFunction;
  const get_bdsp_static_result_columns: AnyFunction;
  const generate_bdsp_static: AnyFunction;
  const get_bdsp_underground_field_groups: AnyFunction;
  const get_bdsp_underground_result_columns: AnyFunction;
  const generate_bdsp_underground: AnyFunction;
  const get_swsh_overworld_field_groups: AnyFunction;
  const get_swsh_overworld_result_columns: AnyFunction;
  const generate_swsh_overworld: AnyFunction;
}

declare module '*.png' {
  const value: any;
  export = value;
}
