pub(crate) use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, anychar, char, digit1, line_ending, one_of, space0, space1},
    combinator::{flat_map, map, map_res, opt},
    error::ErrorKind,
    multi::{fold_many1, many0, many1, separated_list},
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

macro_rules! signed_nr_str_parser {
    ($fn_name: ident, $t:ident) => {
        pub fn $fn_name(s: &str) -> IResult<&str, $t> {
            map_res(
                pair(opt(one_of("+-")), digit1),
                |(sign, s): (Option<char>, &str)| {
                    s.parse::<$t>()
                        .map_err(|_err| Err::Error((s, ErrorKind::Digit)))
                        .map(|v| if let Some('-') = sign { -v } else { v })
                },
            )(s)
        }
    };
}

unsigned_nr_str_parser!(usize_str, usize);
unsigned_nr_str_parser!(u16_str, u16);
unsigned_nr_str_parser!(u32_str, u32);
unsigned_nr_str_parser!(u64_str, u64);

signed_nr_str_parser!(isize_str, isize);
signed_nr_str_parser!(i64_str, i64);
