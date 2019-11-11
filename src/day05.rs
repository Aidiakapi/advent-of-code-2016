use crate::prelude::*;

fn byte_to_hex(b: u8) -> char {
    assert!(b < 16);
    (if b < 10 { b'0' + b } else { b'a' - 10 + b }) as char
}

fn write_u64_to_buffer(buffer: &mut Vec<u8>, mut nr: u64) {
    let start_len = buffer.len();
    loop {
        buffer.push((nr % 10) as u8 + b'0');
        if nr < 10 {
            break;
        }
        nr /= 10;
    }
    (&mut buffer[start_len..]).reverse();
}

pub fn pt1(input: &str) -> Result<String> {
    let mut out = String::with_capacity(8);

    let mut buffer = Vec::with_capacity(input.len() + 10);
    for c in input.chars() {
        buffer.push(c as u8);
    }
    for i in 0u64.. {
        buffer.truncate(input.len());
        write_u64_to_buffer(&mut buffer, i);

        let md5::Digest(bytes) = md5::compute(&buffer);
        if bytes[0] == 0 && bytes[1] == 0 && bytes[2] < 16 {
            out.push(byte_to_hex(bytes[2]));
            if out.len() == 8 {
                return Ok(out);
            }
        }
    }
    unreachable!()
}

pub fn pt2(input: &str) -> Result<String> {
    let mut out = [' '; 8];
    let mut fill_count = 0;

    let mut buffer = Vec::with_capacity(input.len() + 10);
    for c in input.chars() {
        buffer.push(c as u8);
    }
    for i in 0u64.. {
        buffer.truncate(input.len());
        write_u64_to_buffer(&mut buffer, i);

        let md5::Digest(bytes) = md5::compute(&buffer);
        if bytes[0] != 0 || bytes[1] != 0 || bytes[2] >= 8 {
            continue;
        }
        if out[bytes[2] as usize] != ' ' {
            continue;
        }
        out[bytes[2] as usize] = byte_to_hex(bytes[3] >> 4);
        fill_count += 1;
        if fill_count == 8 {
            return Ok(out.iter().collect());
        }
    }
    unreachable!()
}

#[test]
fn day05() -> Result<()> {
    {
        let mut buffer = vec![b'a', b'b', b'c'];
        write_u64_to_buffer(&mut buffer, 1234);
        assert_eq!(buffer, vec![b'a', b'b', b'c', b'1', b'2', b'3', b'4']);
    }

    test_part!(pt1, "abc" => "18f47a30".to_owned());
    test_part!(pt2, "abc" => "05ace8e3".to_owned());

    Ok(())
}
