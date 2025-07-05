
use nom::{
    Parser, IResult,
    branch::alt,
    combinator::{opt, map, map_res, recognize},
    multi::many1,
    sequence::{preceded, delimited},
    character::complete::{one_of, digit1},
    bytes::complete::{tag},
};
use super::*;


/// 式
pub fn expression(src: &str) -> IResult<&str, Expression> {
    alt((
        factor,
    )).parse(src)
}

/// 因子
pub fn factor(src: &str) -> IResult<&str, Expression> {

    let priority = delimited(
        tag("("),
        factor,
        tag(")"),
    );

    alt((
        map(priority, |used| Expression::Priority(Box::new(used))),
        map(literal, |used| Expression::Literal(used)),
    )).parse(src)
}

/// リテラル
pub fn literal(src: &str) -> IResult<&str, Value> {
    alt((
        map(real_literal, |used| Value::Real(used)),
        map(int_literal, |used| Value::Int(used)),
    )).parse(src)
}

/// 整数リテラル
fn int_literal(src: &str) -> IResult<&str, i128> {

    let bin_literal = preceded(
        tag("0b"),
        map_res(recognize(many1(one_of("01"))),
            |used| i128::from_str_radix(used, 2)
        )
    );
    let hex_literal = preceded(
        tag("0x"),
        map_res(recognize(many1(one_of("0123456789abcdefABCDEF"))),
            |used| i128::from_str_radix(used, 16)
        )
    );
    let dec_literal = map_res(recognize(many1(digit1)), |used: &str| {
        used.parse::<i128>()
    });

    alt((
        bin_literal,
        hex_literal,
        dec_literal
    )).parse(src)
}

/// 実数リテラル
fn real_literal(src: &str) -> IResult<&str, f64> {

    let dec_parser = recognize((
        digit1,
        tag("."),
        opt(digit1),
    ));
    let exp_parser = recognize((
        digit1,
        opt(tag(".")),
        opt(digit1),
        tag("e"),
        opt(one_of("+-")),
        digit1,
    ));

    alt((
        map_res(exp_parser, |used: &str| used.parse()),
        map_res(dec_parser, |used: &str| used.parse()),
    )).parse(src)
}
