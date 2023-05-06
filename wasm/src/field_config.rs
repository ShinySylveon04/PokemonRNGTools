use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldComponent {
    id: String,
    label: String,
    default_value: String,
    required: bool,
    r#type: String,
    options: Option<Vec<String>>,
}

impl FieldComponent {
    pub fn label(id: impl ToString, label: impl ToString) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: "".to_string(),
            required: true,
            r#type: "label".to_string(),
            options: None,
        }
    }

    pub fn number(id: impl ToString, label: impl ToString, default_value: Option<u32>) -> Self {
        let default_value = match default_value {
            Some(num) => num.to_string(),
            None => "".to_string(),
        };
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value,
            required: true,
            r#type: "number".to_string(),
            options: None,
        }
    }

    pub fn hex_number(id: impl ToString, label: impl ToString, default_value: Option<u32>) -> Self {
        let default_value = match default_value {
            Some(num) => num.to_string(),
            None => "".to_string(),
        };
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value,
            required: true,
            r#type: "hex_number".to_string(),
            options: None,
        }
    }

    pub fn text(id: impl ToString, label: impl ToString, default_value: impl ToString) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value.to_string(),
            required: true,
            r#type: "text".to_string(),
            options: None,
        }
    }

    pub fn checkbox(id: impl ToString, label: impl ToString) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: "false".to_string(),
            required: true,
            r#type: "checkbox".to_string(),
            options: None,
        }
    }

    pub fn select(id: impl ToString, label: impl ToString, options: &[impl ToString]) -> Self {
        let default_value = options
            .first()
            .map(|string| string.to_string())
            .unwrap_or_default();
        let options = options
            .iter()
            .map(|string| string.to_string())
            .collect::<Vec<String>>();
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value,
            required: true,
            r#type: "select".to_string(),
            options: Some(options),
        }
    }
}

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
