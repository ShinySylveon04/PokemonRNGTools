use chatot_forms::{FieldGroup, LargeComponent};
use console_error_panic_hook::set_once as init_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_field_groups() -> JsValue {
    init_panic_hook();
    let rng_info_components = vec![
        LargeComponent::seed(),
        LargeComponent::min_advances(),
        LargeComponent::max_advances(),
        LargeComponent::delay(),
    ];

    let filer_components = vec![
        LargeComponent::shiny_type(),
        LargeComponent::nature_multiselect(),
    ];

    let field_groups = vec![
        FieldGroup::new("RNG Info", rng_info_components),
        FieldGroup::new("Filters", filer_components),
    ];
    JsValue::from_serde(&field_groups).unwrap()
}

#[wasm_bindgen]
pub fn get_result_columns() -> JsValue {
    init_panic_hook();
    let result_columns = vec!["Advances", "IVs", "PID"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();
    JsValue::from_serde(&result_columns).unwrap()
}

#[wasm_bindgen]
pub fn generate_results(_settings: &JsValue) -> JsValue {
    init_panic_hook();
    let results = vec![
        vec!["0", "31 / 31 / 31 / 31 / 31 / 31", "aabbccdd"],
        vec!["1", "31 / 31 / 31 / 31 / 31 / 31", "aabbccdd"],
        vec!["2", "31 / 31 / 31 / 31 / 31 / 31", "aabbccdd"],
        vec!["3", "31 / 31 / 31 / 31 / 31 / 31", "aabbccdd"],
        vec!["4", "31 / 31 / 31 / 31 / 31 / 31", "aabbccdd"],
        vec!["5", "31 / 31 / 31 / 31 / 31 / 31", "aabbccdd"],
        vec!["6", "31 / 31 / 31 / 31 / 31 / 31", "aabbccdd"],
    ];
    JsValue::from_serde(&results).unwrap()
}
