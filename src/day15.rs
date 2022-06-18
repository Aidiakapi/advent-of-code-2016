use crate::prelude::*;
use num::Integer;

pub fn pt1(input: Vec<Disc>) -> Result<u64> {
    let mut lcm = 1;
    let mut start_time = 0;
    for (idx, disc) in input.into_iter().enumerate() {
        let current = (idx as u64 + 1 + disc.current + start_time) % disc.size;
        let remainder = (disc.size - current) % disc.size;

        // `start_time` must advance `remainder`, but is only allowed to increment
        // in multiples of `lcm` (otherwise previous discs would no longer be valid).
        let mut increment = remainder;
        while increment % lcm != 0 {
            increment += disc.size;
        }
        start_time += increment;
        lcm = lcm.lcm(&disc.size);
    }
    Ok(start_time)
}

pub fn pt2(mut input: Vec<Disc>) -> Result<u64> {
    input.push(Disc {
        size: 11,
        current: 0,
    });
    pt1(input)
}

pub fn parse(s: &str) -> IResult<&str, Vec<Disc>> {
    use parsers::*;

    fold_many1(
        tuple((
            preceded(tag("Disc #"), usize_str),
            delimited(
                tag(" has "),
                u64_str,
                tag(" positions; at time=0, it is at position "),
            ),
            terminated(u64_str, pair(char('.'), opt(line_ending))),
        )),
        || Vec::new(),
        |mut v, (idx, size, current)| {
            v.push(Disc { size, current });
            assert_eq!(idx, v.len());
            v
        },
    )(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Disc {
    size: u64,
    current: u64,
}

#[test]
fn day15() -> Result<()> {
    test_part!(parse, pt1, "\
Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1." => 5);

    Ok(())
}
