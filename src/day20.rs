use crate::prelude::*;
use std::cmp::{Ord, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    from: u32,
    to: u32,
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..={}", self.from, self.to)
    }
}

pub fn pt1(mut ranges: Vec<Range>) -> Result<u32> {
    ranges.sort_unstable();
    let mut lowest_allowed = 0;
    for range in ranges {
        if lowest_allowed >= range.from && lowest_allowed <= range.to {
            lowest_allowed = range.to + 1;
        }
    }
    Ok(lowest_allowed)
}

fn coalesce_ranges(mut old: Vec<Range>) -> Vec<Range> {
    assert!(!old.is_empty());
    old.sort_unstable();
    let mut new = Vec::new();
    new.push(old[0]);

    for &range in old.iter().skip(1) {
        if range.from > range.to {
            continue;
        }
        let last = new.last_mut().unwrap();
        if (range.from as u64) <= (last.to as u64) + 1 {
            // Combine the ranges
            last.to = last.to.max(range.to);
        } else {
            // New range
            new.push(range);
        }
    }

    new
}

pub fn pt2(ranges: Vec<Range>) -> Result<u32> {
    let ranges = coalesce_ranges(ranges);

    let mut total = 0;
    if ranges[0].from != 0 {
        total += ranges[0].from - 1;
    }
    for (a, b) in ranges.iter().zip(ranges.iter().skip(1)) {
        total += b.from - a.to - 1;
    }
    if ranges.last().unwrap().to != std::u32::MAX {
        total += std::u32::MAX - ranges.last().unwrap().to;
    }

    Ok(total)
}

pub fn parse(s: &str) -> IResult<&str, Vec<Range>> {
    use parsers::*;
    separated_list(
        line_ending,
        map(pair(u32_str, preceded(char('-'), u32_str)), |(from, to)| {
            Range { from, to }
        }),
    )(s)
}

#[test]
fn day20() -> Result<()> {
    test_part!(parse, pt2, "\
5-8
0-2
4-7" => std::u32::MAX - 8);

    Ok(())
}
