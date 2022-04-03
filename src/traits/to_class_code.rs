use crate::enums;

pub trait ToClassCode {
    fn to_class_code(&self) -> enums::ClassCode;
}
