use crate::prelude::*;

fn scramble_instr(instr: Instruction, data: &mut [u8]) {
    use Instruction::*;
    match instr {
        SwapPosition(a, b) => data.swap(a, b),
        SwapLetter(a, b) => data.swap(
            data.iter().position(|&c| c == a).unwrap(),
            data.iter().position(|&c| c == b).unwrap(),
        ),
        Rotate(amt) => {
            data.rotate_right((data.len() as isize + amt) as usize % data.len());
        }
        RotateBasedPosition(c) => {
            let idx = data.iter().position(|&v| c == v).unwrap();
            let rot = if idx >= 4 { idx + 2 } else { idx + 1 };
            data.rotate_right(rot % data.len());
        }
        Reverse(a, b) => data[a..=b].reverse(),
        Move(mut a, b) => {
            while a != b {
                if a < b {
                    data.swap(a, a + 1);
                    a += 1;
                } else {
                    data.swap(a, a - 1);
                    a -= 1;
                }
            }
        }
    }
}
fn scramble(instructions: &[Instruction], data: &mut [u8]) {
    for &instr in instructions {
        scramble_instr(instr, data);
    }
}

fn unscramble_instr(instr: Instruction, data: &mut [u8]) {
    use Instruction::*;
    match instr {
        SwapPosition(a, b) => data.swap(a, b),
        SwapLetter(a, b) => data.swap(
            data.iter().position(|&c| c == a).unwrap(),
            data.iter().position(|&c| c == b).unwrap(),
        ),
        Rotate(amt) => {
            data.rotate_left((data.len() as isize + amt) as usize % data.len());
        }
        RotateBasedPosition(c) => {
            debug_assert!(data.len() == 8);
            // Scrambling an 8-len str will cause these rotations
            // 0 rot 1 => 1
            // 1 rot 2 => 3
            // 2 rot 3 => 5
            // 3 rot 4 => 7
            // 4 rot 6 => 10 | 2
            // 5 rot 7 => 12 | 4
            // 6 rot 8 | 0 => 14 | 6
            // 7 rot 9 | 1 => 16 | 0
            const UNSCRAMBLE_TABLE: [usize; 8] = [1, 1, 6, 2, 7, 3, 0, 4];
            let new_pos = data.iter().position(|&v| v == c).unwrap();
            data.rotate_left(UNSCRAMBLE_TABLE[new_pos]);
        }
        Reverse(a, b) => data[a..=b].reverse(),
        Move(b, mut a) => {
            while a != b {
                if a < b {
                    data.swap(a, a + 1);
                    a += 1;
                } else {
                    data.swap(a, a - 1);
                    a -= 1;
                }
            }
        }
    }
}
fn unscramble(instructions: &[Instruction], data: &mut [u8]) {
    for &instr in instructions.iter().rev() {
        unscramble_instr(instr, data);
    }
}

pub fn pt1(input: Vec<Instruction>) -> Result<String> {
    let mut data: Vec<_> = b"abcdefgh".iter().cloned().collect();
    scramble(&input, data.as_mut_slice());
    Ok(String::from_utf8(data).unwrap())
}

pub fn pt2(input: Vec<Instruction>) -> Result<String> {
    let mut data: Vec<_> = b"fbgdceah".iter().cloned().collect();
    unscramble(&input, data.as_mut_slice());
    Ok(String::from_utf8(data).unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    Rotate(isize),
    RotateBasedPosition(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

pub fn parse(s: &str) -> IResult<&str, Vec<Instruction>> {
    use parsers::*;
    let line = alt((
        map(
            pair(
                preceded(tag("swap position "), usize_str),
                preceded(tag(" with position "), usize_str),
            ),
            |(a, b)| Instruction::SwapPosition(a, b),
        ),
        map(
            pair(
                preceded(tag("swap letter "), anychar),
                preceded(tag(" with letter "), anychar),
            ),
            |(a, b)| Instruction::SwapLetter(a as u8, b as u8),
        ),
        map(
            pair(
                preceded(tag("rotate "), alt((tag("left "), tag("right ")))),
                terminated(usize_str, terminated(tag(" step"), opt(char('s')))),
            ),
            |(dir, amt)| Instruction::Rotate(if dir == "right " { 1 } else { -1 } * (amt as isize)),
        ),
        map(
            preceded(tag("rotate based on position of letter "), anychar),
            |c| Instruction::RotateBasedPosition(c as u8),
        ),
        map(
            pair(
                preceded(tag("reverse positions "), usize_str),
                preceded(tag(" through "), usize_str),
            ),
            |(a, b)| Instruction::Reverse(a, b),
        ),
        map(
            pair(
                preceded(tag("move position "), usize_str),
                preceded(tag(" to position "), usize_str),
            ),
            |(a, b)| Instruction::Move(a, b),
        ),
    ));

    separated_list1(line_ending, line)(s)
}

#[test]
fn day21() -> Result<()> {
    fn instr(s: &'static str) -> Instruction {
        match parse(s) {
            Ok(("", v)) => v[0],
            Ok((s, _)) => {
                eprintln!("incomplete parse, remainder: {:?}", s);
                panic!("parser error");
            }
            Err(parsers::Err::Error(e)) | Err(parsers::Err::Failure(e)) => {
                eprintln!("error: {:?}", e);
                panic!("parser errored");
            }
            Err(parsers::Err::Incomplete(_)) => unreachable!(),
        }
    }
    let buf = &mut b"abcde".iter().cloned().collect::<Vec<u8>>();
    // Scramble
    scramble(&[instr("swap position 4 with position 0")], buf);
    assert_eq!(buf, b"ebcda");
    scramble(&[instr("swap letter d with letter b")], buf);
    assert_eq!(buf, b"edcba");
    scramble(&[instr("reverse positions 0 through 4")], buf);
    assert_eq!(buf, b"abcde");
    scramble(&[instr("rotate left 1 step")], buf);
    assert_eq!(buf, b"bcdea");
    scramble(&[instr("move position 1 to position 4")], buf);
    assert_eq!(buf, b"bdeac");
    scramble(&[instr("move position 3 to position 0")], buf);
    assert_eq!(buf, b"abdec");
    scramble(&[instr("rotate based on position of letter b")], buf);
    assert_eq!(buf, b"ecabd");
    scramble(&[instr("rotate based on position of letter d")], buf);
    assert_eq!(buf, b"decab");

    // Unscramble (impossible to unrotate based on position with a 5-len str)
    buf.clear();
    buf.extend(b"abdec".iter().cloned());
    unscramble(&[instr("move position 3 to position 0")], buf);
    assert_eq!(buf, b"bdeac");
    unscramble(&[instr("move position 1 to position 4")], buf);
    assert_eq!(buf, b"bcdea");
    unscramble(&[instr("rotate left 1 step")], buf);
    assert_eq!(buf, b"abcde");
    unscramble(&[instr("reverse positions 0 through 4")], buf);
    assert_eq!(buf, b"edcba");
    unscramble(&[instr("swap letter d with letter b")], buf);
    assert_eq!(buf, b"ebcda");
    unscramble(&[instr("swap position 4 with position 0")], buf);
    assert_eq!(buf, b"abcde");

    Ok(())
}
