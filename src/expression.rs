
use super::*;


/// 式
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// リテラル
    Literal(Value),
    /// 優先
    Priority(Box<Expression>),
}
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(v) => write!(f, "{}", v),
            Self::Priority(v) => write!(f, "({})", v),
        }
    }
}
