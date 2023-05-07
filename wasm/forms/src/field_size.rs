#[derive(Debug, Clone, Copy)]
pub enum FieldSize {
    Small,
    Large,
}

impl ToString for FieldSize {
    fn to_string(&self) -> String {
        match self {
            FieldSize::Small => "small",
            FieldSize::Large => "large",
        }
        .to_string()
    }
}
