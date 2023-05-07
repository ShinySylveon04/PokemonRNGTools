use super::field_size::FieldSize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldComponent {
    id: String,
    label: String,
    default_value: String,
    required: bool,
    r#type: String,
    size: String,
    options: Option<Vec<String>>,
}

impl FieldComponent {
    pub fn label(id: impl ToString, label: impl ToString, size: FieldSize) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: "".to_string(),
            required: true,
            r#type: "label".to_string(),
            options: None,
            size: size.to_string(),
        }
    }

    pub fn number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
        size: FieldSize,
    ) -> Self {
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
            size: size.to_string(),
        }
    }

    pub fn hex_number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
        size: FieldSize,
    ) -> Self {
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
            size: size.to_string(),
        }
    }

    pub fn text(
        id: impl ToString,
        label: impl ToString,
        default_value: impl ToString,
        size: FieldSize,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: default_value.to_string(),
            required: true,
            r#type: "text".to_string(),
            options: None,
            size: size.to_string(),
        }
    }

    pub fn checkbox(id: impl ToString, label: impl ToString, size: FieldSize) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            default_value: "false".to_string(),
            required: true,
            r#type: "checkbox".to_string(),
            options: None,
            size: size.to_string(),
        }
    }

    pub fn select(
        id: impl ToString,
        label: impl ToString,
        options: &[impl ToString],
        size: FieldSize,
    ) -> Self {
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
            size: size.to_string(),
        }
    }
}

pub struct LargeComponent;

impl LargeComponent {
    pub fn label(id: impl ToString, label: impl ToString) -> FieldComponent {
        FieldComponent::label(id, label, FieldSize::Large)
    }

    pub fn number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
    ) -> FieldComponent {
        FieldComponent::number(id, label, default_value, FieldSize::Large)
    }

    pub fn hex_number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
    ) -> FieldComponent {
        FieldComponent::hex_number(id, label, default_value, FieldSize::Large)
    }

    pub fn text(
        id: impl ToString,
        label: impl ToString,
        default_value: impl ToString,
    ) -> FieldComponent {
        FieldComponent::text(id, label, default_value, FieldSize::Large)
    }

    pub fn checkbox(id: impl ToString, label: impl ToString) -> FieldComponent {
        FieldComponent::checkbox(id, label, FieldSize::Large)
    }

    pub fn select(
        id: impl ToString,
        label: impl ToString,
        options: &[impl ToString],
    ) -> FieldComponent {
        FieldComponent::select(id, label, options, FieldSize::Large)
    }
}

pub struct SmallComponent;

impl SmallComponent {
    pub fn label(id: impl ToString, label: impl ToString) -> FieldComponent {
        FieldComponent::label(id, label, FieldSize::Small)
    }

    pub fn number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
    ) -> FieldComponent {
        FieldComponent::number(id, label, default_value, FieldSize::Small)
    }

    pub fn hex_number(
        id: impl ToString,
        label: impl ToString,
        default_value: Option<u32>,
    ) -> FieldComponent {
        FieldComponent::hex_number(id, label, default_value, FieldSize::Small)
    }

    pub fn text(
        id: impl ToString,
        label: impl ToString,
        default_value: impl ToString,
    ) -> FieldComponent {
        FieldComponent::text(id, label, default_value, FieldSize::Small)
    }

    pub fn checkbox(id: impl ToString, label: impl ToString) -> FieldComponent {
        FieldComponent::checkbox(id, label, FieldSize::Small)
    }

    pub fn select(
        id: impl ToString,
        label: impl ToString,
        options: &[impl ToString],
    ) -> FieldComponent {
        FieldComponent::select(id, label, options, FieldSize::Small)
    }
}
