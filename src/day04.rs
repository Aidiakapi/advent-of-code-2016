use crate::prelude::*;

impl Room {
    fn compute_checksum(&self) -> String {
        self.name
            .iter()
            .flat_map(|v| v.chars())
            .sorted()
            .group_by(|c| *c)
            .into_iter()
            .map(|(c, group)| (c, group.count()))
            .sorted_by(|(ac, acount), (bc, bcount)| bcount.cmp(acount).then(ac.cmp(bc)))
            .take(5)
            .map(|(c, _)| c)
            .collect()
    }

    fn decrypt_name(&self) -> String {
        let rotation = (self.sector_id % 26) as u8;
        self.name
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| (((c as u8) - b'a' + rotation) % 26 + b'a') as char)
                    .collect::<String>()
            })
            .join(" ")
    }
}

pub fn pt1(input: Vec<Room>) -> Result<u32> {
    Ok(input
        .into_iter()
        .filter(|room| room.compute_checksum() == room.checksum)
        .map(|room| room.sector_id)
        .sum())
}

pub fn pt2(input: Vec<Room>) -> Result<u32> {
    input
        .into_iter()
        .filter(|room| room.compute_checksum() == room.checksum)
        .filter(|room| room.decrypt_name() == "northpole object storage")
        .next()
        .map(|room| room.sector_id)
        .ok_or(anyhow!("no north pole objects room found"))
}

pub fn parse(s: &str) -> IResult<&str, Vec<Room>> {
    use parsers::*;
    let room = map(
        tuple((
            separated_list(char('-'), alpha1),
            preceded(char('-'), u32str),
            terminated(preceded(char('['), alpha1), char(']')),
        )),
        |(name, sector_id, checksum)| Room {
            name: Vec::from_iter(name.into_iter().map(str::to_owned)),
            sector_id,
            checksum: checksum.to_owned(),
        },
    );

    separated_list(char('\n'), room)(s)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Room {
    name: Vec<String>,
    sector_id: u32,
    checksum: String,
}

#[test]
fn day04() -> Result<()> {
    test_parse!(parse, "aaaaa-bbb-z-y-x-123[abxyz]" => vec![Room {
        name: vec!["aaaaa".to_owned(), "bbb".to_owned(), "z".to_owned(), "y".to_owned(), "x".to_owned()],
        sector_id: 123,
        checksum: "abxyz".to_owned(),
    }]);

    test_part!(parse, pt1, "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]" => 1514);

    assert_eq!(
        Room {
            name: vec!["qzmt".to_owned(), "zixmtkozy".to_owned(), "ivhz".to_owned()],
            sector_id: 343,
            checksum: String::new(),
        }
        .decrypt_name(),
        "very encrypted name".to_owned()
    );

    Ok(())
}
