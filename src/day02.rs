use crate::prelude::*;

pub fn pt1(input: Vec<Vec<Direction>>) -> Result<String> {
    let mut x = 1u8;
    let mut y = 1u8;
    let mut out = String::new();
    for row in input {
        for cmd in row {
            use Direction::*;
            match cmd {
                Up if y > 0 => y -= 1,
                Down if y < 2 => y += 1,
                Left if x > 0 => x -= 1,
                Right if x < 2 => x += 1,
                _ => {}
            }
        }
        out.push((x + y * 3 + '1' as u8) as char);
    }

    Ok(out)
}

pub fn pt2(input: Vec<Vec<Direction>>) -> Result<String> {
    const KEYPAD: &'static [u8] =b"\
-------\
---1---\
--234--\
-56789-\
--ABC--\
---D---\
-------";
    let mut idx: isize = 7 * 3 + 1;
    let mut out = String::new();
    for row in input {
        for cmd in row {
            use Direction::*;
            let new_idx = idx
                + match cmd {
                    Up => -7,
                    Down => 7,
                    Left => -1,
                    Right => 1,
                };
            if KEYPAD[new_idx as usize] != b'-' {
                idx = new_idx;
            }
        }
        out.push(KEYPAD[idx as usize] as char);
    }

    Ok(out)
}

pub fn parse(s: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    use parsers::*;
    let read_dir = map_res(anychar, |c| {
        Ok(match c {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => return Err(()),
        })
    });
    separated_list1(tag("\n"), many1(read_dir))(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[test]
fn day02() -> Result<()> {
    test_parse!(parse, r"
ULL
RRDDD"
        => vec![
            vec![Direction::Up, Direction::Left, Direction::Left],
            vec![Direction::Right, Direction::Right, Direction::Down, Direction::Down, Direction::Down],
        ]
    );

    const TEST_INPUT: &'static str = r"
ULL
RRDDD
LURDL
UUUUD";
    test_part!(parse, pt1, TEST_INPUT => "1985");
    test_part!(parse, pt2, TEST_INPUT => "5DB3");

    Ok(())
}
