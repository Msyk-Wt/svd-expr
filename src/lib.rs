
use nom::{
    Parser, IResult,
    branch::alt,
    combinator::{map_res, recognize},
    multi::many1,
    sequence::preceded,
    character::complete::{one_of, digit1},
    bytes::complete::{tag},
};


/// 式
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// リテラル
    Literal(Value),
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(v) => write!(f, "{}", v),
        }
    }
}


/// 数値
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// 整数
    Int(i128),
    /// 実数
    Real(f64),
}
impl Value {

    pub fn parse_literal(src: &str) -> IResult<&str, Self> {
        alt((
            Self::__parse_int,
        )).parse(src)
    }

    fn __parse_int(src: &str) -> IResult<&str, Self> {

        let bin_parser = preceded(
            tag("0b"),
            map_res(recognize(many1(one_of("01"))),
                |used| i128::from_str_radix(used, 2)
            )
        );

        let hex_parser = preceded(
            tag("0x"),
            map_res(recognize(many1(one_of("0123456789abcdefABCDEF"))),
                |used| i128::from_str_radix(used, 16)
            )
        );

        let dec_parser = map_res(digit1,
            |used| i128::from_str_radix(used, 10)
        );

        let (unused, used) = alt((
            bin_parser, hex_parser, dec_parser
        )).parse(src)?;

        Ok((unused, Self::Int(used)))
    }

}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Real(v) => write!(f, "{}", v),
        }
    }
}
