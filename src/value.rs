
/// 値
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// 整数
    Int(i128),
    /// 実数
    Real(f64),
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Real(v) => write!(f, "{}", v),
        }
    }
}
