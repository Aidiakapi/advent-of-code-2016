use crate::prelude::*;

fn decompress(s: &str) -> Result<String> {
    use parsers::*;

    #[derive(Debug, Clone)]
    enum Section<'s> {
        Text(&'s str),
        Repetition(usize, &'s str),
    }
    let section = alt((
        map(alpha1, |s: &str| Section::Text(s)),
        map(
            flat_map(
                terminated(
                    preceded(char('('), pair(usize_str, preceded(char('x'), usize_str))),
                    char(')'),
                ),
                |(length, count)| map(take(length), move |substr: &str| (count, substr)),
            ),
            |(count, substr)| Section::Repetition(count, substr),
        ),
    ));
    let (_, sections) =
        many1(section)(s).map_err(|_| anyhow!("invalid characters while decompressing"))?;
    let mut out = String::new();
    for section in sections {
        match section {
            Section::Text(s) => out.push_str(s),
            Section::Repetition(count, s) => {
                for _ in 0..count {
                    out.push_str(s);
                }
            }
        }
    }

    Ok(out)
}

pub fn pt1(input: &str) -> Result<usize> {
    Ok(decompress(input)?.len())
}

pub fn pt2(input: &str) -> Result<u64> {
    use parsers::*;

    #[derive(Debug, Clone)]
    enum Section<'s> {
        Text(u64),
        Repetition(u64, &'s str),
    }

    fn parse_sections(s: &str) -> Result<Vec<Section>> {
        let (_, sections) = many0(alt((
            map(alpha1, |s: &str| Section::Text(s.len() as u64)),
            map(
                flat_map(
                    terminated(
                        preceded(char('('), pair(u64_str, preceded(char('x'), u64_str))),
                        char(')'),
                    ),
                    |(length, count)| map(take(length), move |substr: &str| (count, substr)),
                ),
                |(count, substr)| Section::Repetition(count, substr),
            ),
        )))(s)
        .map_err(|_| anyhow!("invalid characters while decompressing"))?;
        Ok(sections)
    }
    
    fn calc_len(input: &str) -> Result<u64> {
        let mut sum = 0;
        for section in parse_sections(input)? {
            sum += match section {
                Section::Text(len) => len,
                Section::Repetition(count, substr) => count * calc_len(substr)?,
            };
        }
        Ok(sum)
    }

    Ok(calc_len(input)?)
}

#[test]
fn day09() -> Result<()> {
    assert_eq!(&decompress("ADVENT")?, "ADVENT");
    assert_eq!(&decompress("A(1x5)BC")?, "ABBBBBC");
    assert_eq!(&decompress("(3x3)XYZ")?, "XYZXYZXYZ");
    assert_eq!(&decompress("A(2x2)BCD(2x2)EFG")?, "ABCBCDEFEFG");
    assert_eq!(&decompress("(6x1)(1x3)A")?, "(1x3)A");
    assert_eq!(&decompress("X(8x2)(3x3)ABCY")?, "X(3x3)ABC(3x3)ABCY");

    test_part!(pt2,
        "(27x12)(20x12)(13x14)(7x10)(1x12)A" => 241920,
        "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN" => 445
    );

    Ok(())
}
