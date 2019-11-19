use crate::prelude::*;

pub fn pt1(input: u32) -> Result<u32> {
    if input == 0 {
        return Err(anyhow!("0 is invalid"));
    }
    // The sequence is all positive odd numbers consecutively, but resets
    // every time a power of two is encountered.
    // Consequently, if you subtract the highest power of two, what remains
    // is an index into a sequence of all the positive odd numbers.
    let idx = input & !(1 << 31 - input.leading_zeros());
    Ok(idx * 2 + 1)
}

pub fn pt2(input: u32) -> Result<u32> {
    if input == 0 {
        return Err(anyhow!("0 is invalid"));
    }
    if input >= 3486784402u32 {
        return Err(anyhow!("input may not be 3486784402 or larger"));
    }

    // The sequence resets to 1 at certain points (3^n+1).
    // Between these reset points, for the first half, it
    // increments by 1 for every elf that is added, for the
    // letter half, it increments by 2.
    let (reset_point, next_reset_point) = {
        let mut last_result = 1;
        let mut n = 0;
        loop {
            let reset_point = 3u32.pow(n) + 1;
            if reset_point > input {
                break (last_result, reset_point);
            }
            last_result = reset_point;
            n += 1;
        }
    };
    let span = next_reset_point - reset_point - 1;
    let offset = input - reset_point;
    Ok(if offset <= span / 2 {
        offset + 1
    } else {
        offset + (offset - span / 2) + 1
    })
}

// This is an easy and naive implementation used to
// generate the sequence initially for analysis.
//
// pub fn pt2_naive(input: u32) -> Result<u32> {
//     if input == 0 {
//         return Err(anyhow!("0 is invalid"));
//     }
//     let mut v = Vec::from_iter(1..=input);
//     let mut idx = 0;
//     while v.len() > 2 {
//         let rm_idx = (idx + v.len() / 2) % v.len();
//         v.remove(rm_idx);
//         if rm_idx > idx {
//             idx += 1;
//         }
//         idx %= v.len();
//     }
//     Ok(v[idx])
// }

pub fn parse(s: &str) -> IResult<&str, u32> {
    use parsers::*;
    u32_str(s)
}

#[test]
fn day19() -> Result<()> {
    test_part!(pt1,
        1 => 1,
        2 => 1,
        3 => 3,
        4 => 1,
        5 => 3,
        6 => 5,
        7 => 7,
        8 => 1,
        9 => 3,
        10 => 5,
        11 => 7,
        12 => 9,
        13 => 11,
        14 => 13,
        15 => 15,
        16 => 1,
        17 => 3,
        18 => 5,
        19 => 7,
        20 => 9,
    );

    test_part!(pt2,
        1 => 1,
        2 => 1,
        3 => 3,
        4 => 1,
        5 => 2,
        6 => 3,
        7 => 5,
        8 => 7,
        9 => 9,
        10 => 1,
        11 => 2,
        12 => 3,
        13 => 4,
        14 => 5,
        15 => 6,
        16 => 7,
        17 => 8,
        18 => 9,
        19 => 11,
        20 => 13,
        21 => 15,
        22 => 17,
        23 => 19,
        24 => 21,
        25 => 23,
        26 => 25,
        27 => 27,
        28 => 1,
    );

    Ok(())
}
