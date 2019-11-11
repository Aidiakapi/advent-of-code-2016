use crate::prelude::*;

pub fn pt1(input: Vec<(u32, u32, u32)>) -> Result<usize> {
    Ok(input
        .into_iter()
        .filter(|&(a, b, c)| a + b > c && a + c > b && b + c > a)
        .count())
}

pub fn pt2(input: Vec<(u32, u32, u32)>) -> Result<usize> {
    let mut new_input = Vec::new();
    for i in 0..input.len() / 3 {
        let a = input[i * 3 + 0];
        let b = input[i * 3 + 1];
        let c = input[i * 3 + 2];
        new_input.push((a.0, b.0, c.0));
        new_input.push((a.1, b.1, c.1));
        new_input.push((a.2, b.2, c.2));
    }

    pt1(new_input)
}

pub fn parse(s: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    use parsers::*;
    separated_list(
        tag("\n"),
        tuple((
            preceded(space0, u32str),
            preceded(space1, u32str),
            preceded(space1, u32str),
        )),
    )(s)
}

#[test]
fn day03() -> Result<()> {
    test_parse!(parse, "\
  541  588  421
  827  272  126" => vec![(541, 588, 421), (827, 272, 126)]);
    Ok(())
}
