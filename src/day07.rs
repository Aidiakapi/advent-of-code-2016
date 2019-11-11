use crate::prelude::*;

pub fn pt1(input: Vec<Ip7>) -> Result<usize> {
    Ok(input
        .into_iter()
        .filter(|ip7| {
            let mut any_abba = false;
            for part in ip7 {
                if !part.has_abba() {
                    continue;
                }
                match part.net_type {
                    NetType::Supernet => any_abba = true,
                    NetType::Hypernet => return false,
                }
            }
            any_abba
        })
        .count())
}

impl Ip7Part {
    fn has_abba(&self) -> bool {
        let data = self.data.as_bytes();
        for i in 0..=data.len() - 4 {
            if data[i] == data[i + 3] && data[i + 1] == data[i + 2] && data[i] != data[i + 1] {
                return true;
            }
        }
        false
    }
}

pub fn pt2(input: Vec<Ip7>) -> Result<usize> {
    Ok(input.into_iter().filter(supports_ssl).count())
}

fn is_aba(slice: &[u8]) -> Option<(u8, u8)> {
    assert!(slice.len() == 3);
    if slice[0] == slice[2] && slice[0] != slice[1] {
        Some((slice[0], slice[1]))
    } else {
        None
    }
}

fn supports_ssl(ip: &Ip7) -> bool {
    #[rustfmt::skip] let supernets: Vec<_> = ip.iter().filter_map(
        |x| if x.net_type == NetType::Supernet { Some(&x.data) } else { None }).collect();
    #[rustfmt::skip] let hypernets: Vec<_> = ip.iter().filter_map(
        |x| if x.net_type == NetType::Hypernet { Some(&x.data) } else { None }).collect();
    let mut abas: Vec<(u8, u8)> = Vec::new();
    for supernet in &supernets {
        let supernet = supernet.as_bytes();
        abas.clear();
        abas.extend((0..=supernet.len() - 3).filter_map(|i| is_aba(&supernet[i..i + 3])));

        if abas.len() == 0 {
            continue;
        }
        for hypernet in &hypernets {
            let hypernet = hypernet.as_bytes();
            for (b2, a2) in (0..=hypernet.len() - 3).filter_map(|i| is_aba(&hypernet[i..i + 3])) {
                for &(a1, b1) in &abas {
                    if a1 == a2 && b1 == b2 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn parse(s: &str) -> IResult<&str, Vec<Ip7>> {
    use parsers::*;
    let part = alt((
        map(alpha1, |v: &str| Ip7Part {
            net_type: NetType::Supernet,
            data: v.to_owned(),
        }),
        delimited(
            char('['),
            map(alpha1, |v: &str| Ip7Part {
                net_type: NetType::Hypernet,
                data: v.to_owned(),
            }),
            char(']'),
        ),
    ));

    separated_list(line_ending, many1(part))(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetType {
    Supernet,
    Hypernet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ip7Part {
    net_type: NetType,
    data: String,
}

pub type Ip7 = Vec<Ip7Part>;

#[test]
fn day07() -> Result<()> {
    test_parse!(parse, "ab[cd]ef\n[a][b]cde" => vec![
        vec![
            Ip7Part{ net_type: NetType::Supernet, data: "ab".to_owned() },
            Ip7Part{ net_type: NetType::Hypernet, data: "cd".to_owned() },
            Ip7Part{ net_type: NetType::Supernet, data: "ef".to_owned() },
        ],
        vec![
            Ip7Part{ net_type: NetType::Hypernet, data: "a".to_owned() },
            Ip7Part{ net_type: NetType::Hypernet, data: "b".to_owned() },
            Ip7Part{ net_type: NetType::Supernet, data: "cde".to_owned() },
        ]
    ]);

    test_part!(parse, pt1,
        "abba[mnop]qrst" => 1,
        "abcd[bddb]xyyx" => 0,
        "aaaa[qwer]tyui" => 0,
        "ioxxoj[asdfgh]zxcvbn" => 1,
    );
    test_part!(parse, pt2,
        "aba[bab]xyz" => 1,
        "xyx[xyx]xyx" => 0,
        "aaa[kek]eke" => 1,
        "zazbz[bzb]cdb" => 1,
    );

    Ok(())
}
