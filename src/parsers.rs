pub use nom::{
    bytes::complete::tag,
    character::complete::{anychar, alpha1, char, digit1, one_of},
    combinator::{map, map_res},
    error::ErrorKind,
    multi::{fold_many0, fold_many1, many1, separated_list},
    sequence::{pair, preceded, terminated, tuple},
    Err, IResult,
};

pub fn u32str(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| {
        s.parse::<u32>()
            .map_err(|_err| Err::Error((s, ErrorKind::Verify)))
    })(s)
}

pub fn space0(s: &str) -> IResult<&str, ()> {
    fold_many0(char(' '), (), |_, _| ())(s)
}
pub fn space1(s: &str) -> IResult<&str, ()> {
    fold_many1(char(' '), (), |_, _| ())(s)
}
