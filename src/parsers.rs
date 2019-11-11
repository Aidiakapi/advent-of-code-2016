pub use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1, one_of},
    combinator::{map, map_res},
    error::ErrorKind,
    multi::{many1, separated_list},
    sequence::pair,
    Err, IResult,
};

pub fn u32str(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| {
        s.parse::<u32>()
            .map_err(|_err| Err::Error((s, ErrorKind::Verify)))
    })(s)
}
