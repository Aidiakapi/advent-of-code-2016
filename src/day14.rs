use crate::prelude::*;
use md5::Digest;

fn solve<F>(input: &str, mut f: F) -> u64
where
    F: FnMut(&[u8]) -> Digest,
{
    let mut buffer = Vec::with_capacity(input.len() + 20);
    buffer.extend_from_slice(input.as_bytes());

    let mut index_to_count_next = [0; 16];
    let mut three_repetitions: Vec<(u64, u8)> = Vec::new();
    let mut valid_keys = Vec::with_capacity(128);
    let mut remainder = std::u64::MAX;
    for i in 0u64.. {
        remainder -= 1;
        if remainder == 0 {
            break;
        }
        buffer.truncate(input.len());
        crate::day05::write_u64_to_buffer(&mut buffer, i);
        let Digest(bytes) = f(&buffer);

        let five_repeated_chars = bytes
            .iter()
            .take(14)
            .zip(bytes.iter().skip(1))
            .zip(bytes.iter().skip(2))
            .filter_map(|((a, b), c)| {
                let d = a & 0x0f;
                if d == (b >> 4)
                    && d == (b & 0x0f)
                    && d == (c >> 4)
                    && (d == (a >> 4) || d == (c & 0x0f))
                {
                    Some(d)
                } else {
                    None
                }
            })
            .next();
        if let Some(repeated_char) = five_repeated_chars {
            valid_keys.extend(
                three_repetitions
                    .iter()
                    .skip(index_to_count_next[repeated_char as usize])
                    .filter(|&&(previous_index, previous_repeated_char)| {
                        previous_index + 1000 >= i && previous_repeated_char == repeated_char
                    })
                    .map(|(previous_index, _)| *previous_index),
            );
            if valid_keys.len() > 64 {
                remainder = 1000;
            }

            index_to_count_next[repeated_char as usize] = three_repetitions.len();
        }

        let three_repeated_chars = bytes
            .iter()
            .take(15)
            .zip(bytes.iter().skip(1))
            .filter_map(|(a, b)| {
                let c = a & 0x0f;
                if (c == (a >> 4) && c == (b >> 4)) || (c == (b >> 4) && c == (b & 0x0f)) {
                    Some(c)
                } else {
                    None
                }
            })
            .next();
        if let Some(repeated_char) = three_repeated_chars {
            three_repetitions.push((i, repeated_char));
        }
    }

    valid_keys.sort();
    valid_keys[63]
}

pub fn pt1(input: &str) -> Result<u64> {
    Ok(solve(input, |v| md5::compute(v)))
}

pub fn pt2(input: &str) -> Result<u64> {
    let mut digest_buf = [0; 32];
    Ok(solve(input, |v| {
        let mut current_digest = md5::compute(v);
        for _ in 0..2016 {
            for i in 0..16 {
                digest_buf[i * 2 + 0] = crate::day05::byte_to_hex(current_digest[i] >> 4) as u8;
                digest_buf[i * 2 + 1] = crate::day05::byte_to_hex(current_digest[i] & 0x0f) as u8;
            }
            current_digest = md5::compute(&digest_buf);
        }
        current_digest
    }))
}

#[test]
#[ignore]
fn day14() -> Result<()> {
    test_part!(pt1, "abc" => 22728);
    test_part!(pt2, "abc" => 22551);

    Ok(())
}
