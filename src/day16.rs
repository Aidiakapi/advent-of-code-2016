use crate::prelude::*;
use bitvec::prelude::*;

fn solve(disc_size: usize, seed: BitVec) -> Result<String> {
    if disc_size % 2 != 0 {
        return Err(anyhow!("disc size must be even"));
    }
    let mut buf: BitVec = seed;
    let mut double_buf: BitVec = BitVec::with_capacity(disc_size * 2);
    buf.reserve(disc_size * 2);

    // Dragon curve-like expansion of data
    while buf.len() < disc_size {
        let len = buf.len();
        double_buf.extend(buf.iter());
        double_buf.push(false);
        double_buf.append(&mut buf);
        (!&mut double_buf[len + 1..]).reverse();
        buf.clear();
        std::mem::swap(&mut buf, &mut double_buf);
    }
    buf.truncate(disc_size);
    std::mem::drop(double_buf);

    // Checksum
    loop {
        for i in 0..buf.len() / 2 {
            let checksum_digit = buf[i * 2 + 0] == buf[i * 2 + 1];
            buf.as_mut_bitslice().set(i, checksum_digit);
        }
        buf.truncate(buf.len() / 2);

        if buf.len() % 2 == 1 {
            break;
        }
    }

    Ok(buf.into_iter().map(|b| if b { '1' } else { '0' }).collect())
}

pub fn pt1(input: BitVec) -> Result<String> {
    solve(272, input)
}

pub fn pt2(input: BitVec) -> Result<String> {
    solve(35651584, input)
}

pub fn parse(s: &str) -> IResult<&str, BitVec> {
    use parsers::*;
    fold_many1(one_of("01"), || BitVec::with_capacity(s.len()), |mut v, c| {
        v.push(c == '1');
        v
    })(s)
}

#[test]
fn day16() -> Result<()> {
    test_part!(parse, |input| solve(20, input), "10000" => "01100");

    Ok(())
}
