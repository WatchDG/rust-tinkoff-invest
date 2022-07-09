use crate::types;

#[derive(Debug, Clone)]
pub struct Positions {
    money: Vec<types::Money>,
    blocked_money: Vec<types::Money>,
}
