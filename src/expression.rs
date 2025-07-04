
use super::*;


/// 式
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// リテラル
    Literal(Value),
}
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(v) => write!(f, "{}", v),
        }
    }
}
