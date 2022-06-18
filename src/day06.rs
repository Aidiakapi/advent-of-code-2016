use crate::prelude::*;

fn pt_impl(input: Vec<String>, count_multiplier: i32) -> Result<String> {
    let mut iter = input.iter();
    let len = iter.next().unwrap().len();
    assert!(iter.all(|c| c.len() == len));

    let mut out = String::with_capacity(len);
    let mut counts = HashMap::new();
    for i in 0..len {
        counts.clear();
        for s in &input {
            (*counts.entry(s.as_bytes()[i]).or_insert(0i32)) += 1;
        }
        let (c, _) = counts.iter().max_by_key(|(_, cnt)| *cnt * count_multiplier).unwrap();
        out.push(*c as char);
    }

    Ok(out)

}

pub fn pt1(input: Vec<String>) -> Result<String> {
    pt_impl(input, 1)
}
pub fn pt2(input: Vec<String>) -> Result<String> {
    pt_impl(input, -1)
}

pub fn parse(s: &str) -> IResult<&str, Vec<String>> {
    use parsers::*;
    separated_list1(line_ending, map(alpha1, |v: &str| v.to_owned()))(s)
}

#[test]
fn day06() -> Result<()> {
    test_parse!(parse, "abcdef\nghijkl" => vec!["abcdef".to_owned(), "ghijkl".to_owned()]);

    const EXAMPLE: &'static str = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    test_part!(parse, pt1, EXAMPLE => "easter".to_owned());
    test_part!(parse, pt2, EXAMPLE => "advent".to_owned());

    Ok(())
}
