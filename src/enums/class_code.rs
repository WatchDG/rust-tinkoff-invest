#[derive(Debug, Clone, PartialEq)]
pub enum ClassCode {
    CETS,
    ClassCode(String),
}

impl From<String> for ClassCode {
    fn from(class_code: String) -> Self {
        match class_code.to_lowercase().as_ref() {
            "cets" => ClassCode::CETS,
            _ => ClassCode::ClassCode(class_code),
        }
    }
}
