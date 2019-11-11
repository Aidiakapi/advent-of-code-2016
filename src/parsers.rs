pub use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1, line_ending, one_of, space0, space1},
    combinator::{map, map_res},
    error::ErrorKind,
    multi::{fold_many0, fold_many1, many1, separated_list},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Err, IResult,
};

pub fn u32str(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| {
        s.parse::<u32>()
            .map_err(|_err| Err::Error((s, ErrorKind::Verify)))
    })(s)
}
