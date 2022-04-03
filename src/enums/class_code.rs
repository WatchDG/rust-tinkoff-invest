#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassCode {
    CETS,
    TQBR,
    ClassCode(String),
}

impl From<String> for ClassCode {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_ref() {
            "CETS" => ClassCode::CETS,
            "TQBR" => ClassCode::TQBR,
            _ => ClassCode::ClassCode(value),
        }
    }
}

impl From<ClassCode> for String {
    fn from(value: ClassCode) -> Self {
        match value {
            ClassCode::CETS => "CETS".into(),
            ClassCode::TQBR => "TQBR".into(),
            ClassCode::ClassCode(class_code) => class_code,
        }
    }
}
