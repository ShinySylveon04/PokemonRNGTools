use super::component::FieldComponent;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FieldGroup {
    label: String,
    components: Vec<FieldComponent>,
}

impl FieldGroup {
    pub fn new(label: impl ToString, components: Vec<FieldComponent>) -> Self {
        Self {
            label: label.to_string(),
            components,
        }
    }
}
