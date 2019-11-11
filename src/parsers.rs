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

macro_rules! unsigned_nr_str_parser {
    ($fn_name: ident, $t:ident) => {
        pub fn $fn_name(s: &str) -> IResult<&str, $t> {
            map_res(digit1, |s: &str| {
                s.parse::<$t>()
                    .map_err(|_err| Err::Error((s, ErrorKind::Digit)))
            })(s)
        }
    };
}

unsigned_nr_str_parser!(usize_str, usize);
unsigned_nr_str_parser!(u32_str, u32);
