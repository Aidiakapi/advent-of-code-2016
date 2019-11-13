use crate::prelude::*;
use std::fmt::{Debug, Formatter, Result as fmtResult, Write};

pub fn pt1(input: Vec<Vec<Module>>) -> Result<usize> {
    use std::{collections::VecDeque, rc::Rc};

    let facility = Facility::new(input);
    assert!(facility.is_safe_configuration());
    struct TrackedFacility {
        parent: Option<Rc<TrackedFacility>>,
        current: Facility,
    }
    impl TrackedFacility {
        pub fn depth(&self) -> usize {
            match self.parent.as_ref() {
                Some(parent) => parent.depth() + 1,
                None => 0,
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert(facility.clone());
    let mut dfs = VecDeque::new();
    dfs.push_back(Rc::new(TrackedFacility {
        parent: None,
        current: facility,
    }));

    while let Some(tracked) = dfs.pop_front() {
        for facility in tracked.current.next_configurations() {
            if !visited.insert(facility.clone()) {
                continue;
            }
            let child = Rc::new(TrackedFacility {
                parent: Some(Rc::clone(&tracked)),
                current: facility,
            });
            if child.current.is_solved() {
                let depth = child.depth();
                // let mut current = Some(&child);
                // let mut current_depth = depth as isize;
                // while let Some(tracked) = current {
                //     println!("depth {}\n{:#?}\n", current_depth, tracked.current);
                //     current_depth -= 1;
                //     current = tracked.parent.as_ref();
                // }
                return Ok(depth);
            }
            dfs.push_back(child);
        }
    }

    Err(anyhow!("no solution found"))
}

pub fn pt2(mut input: Vec<Vec<Module>>) -> Result<usize> {
    input[0].push(Module::Generator("elerium"));
    input[0].push(Module::Microchip("elerium"));
    input[0].push(Module::Generator("dilithium"));
    input[0].push(Module::Microchip("dilithium"));
    pt1(input)
}

type Storage = u16;
const HALF_SIZE: usize = 8;
const LOW_BITS: Storage = 0x00ff;
const HIGH_BITS: Storage = 0xff00;
const FLOOR_COUNT: usize = 4;
#[derive(Clone, PartialEq, Eq, Hash)]
struct Facility {
    // higher half bits == generators
    // lower half bits == microchips
    floors: [Storage; FLOOR_COUNT],
    chip_count: usize,
    elevator_at: usize,
}

impl Facility {
    fn new(input: Vec<Vec<Module>>) -> Facility {
        let mut floors = [0; FLOOR_COUNT];
        let mut names = HashMap::new();
        let mut gen_count = 0;
        let mut mch_count = 0;
        for (floor, module) in input
            .iter()
            .enumerate()
            .flat_map(|(idx, floor)| repeat(idx).zip(floor.iter()))
        {
            let name = match module {
                Module::Generator(name) => {
                    gen_count += 1;
                    name
                }
                Module::Microchip(name) => {
                    mch_count += 1;
                    name
                }
            };
            let names_len = names.len();
            let idx = *names.entry(name).or_insert(names_len);
            match module {
                Module::Generator(_) => floors[floor] |= 1 << (idx + HALF_SIZE),
                Module::Microchip(_) => floors[floor] |= 1 << idx,
            }
        }
        assert_eq!(gen_count, names.len());
        assert_eq!(gen_count, mch_count);
        assert!(gen_count <= HALF_SIZE);
        Facility {
            floors,
            chip_count: mch_count,
            elevator_at: 0,
        }
    }

    fn is_solved(&self) -> bool {
        (0..FLOOR_COUNT - 1).all(|i| self.floors[i] == 0)
    }

    fn is_safe_floor(floor: Storage) -> bool {
        // The floor is safe if there are either no generators
        (floor & HIGH_BITS) == 0 ||
            // or every module has a matching generator.
            (floor & LOW_BITS) & !(floor >> HALF_SIZE) == 0
    }

    fn is_safe_configuration(&self) -> bool {
        self.floors.iter().cloned().all(Self::is_safe_floor)
    }

    fn next_configurations<'s>(&'s self) -> impl Iterator<Item = Facility> + 's {
        use std::iter::ExactSizeIterator;
        #[derive(Clone, Copy)]
        struct NextFloors {
            a: usize,
            b: usize,
            l: u8,
            c: u8,
        }
        impl Iterator for NextFloors {
            type Item = usize;
            fn next(&mut self) -> Option<Self::Item> {
                if self.c >= self.l {
                    return None;
                }
                self.c += 1;
                Some(if self.c == 1 { self.a } else { self.b })
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let l = self.len();
                (l, Some(l))
            }
        }
        impl ExactSizeIterator for NextFloors {
            fn len(&self) -> usize {
                (self.l - self.c) as usize
            }
        }

        #[rustfmt::skip] let next_floors =
            // if we cannot or shouldn't move down
            if self.elevator_at == 0 || self.floors[0..self.elevator_at].iter().all(|&floor| floor == 0)
        {
            if self.elevator_at == FLOOR_COUNT - 1 {
                NextFloors { a: 0, b: 0, l: 0, c: 0 }
            } else {
                NextFloors { a: self.elevator_at + 1, b: 0, l: 1, c: 0 }
            }
        } else {
            if self.elevator_at == FLOOR_COUNT - 1 {
                NextFloors { a: self.elevator_at - 1, b: 0, l: 1, c: 0 }
            } else {
                NextFloors { a: self.elevator_at + 1, b: self.elevator_at - 1, l: 2, c: 0 }
            }
        };

        let current_floor = self.floors[self.elevator_at];
        let moveable_items = (0..self.chip_count)
            .chain(HALF_SIZE..HALF_SIZE + self.chip_count)
            .filter(move |idx| (current_floor >> idx) & 1 == 1);

        let moveable_pairs = moveable_items
            .clone()
            .enumerate()
            .flat_map(move |(count, a)| {
                repeat(None)
                    .take(1)
                    .chain(moveable_items.clone().skip(count + 1).map(Option::Some))
                    .map(move |b| (a, b))
            });

        moveable_pairs
            .flat_map(move |(a, b)| next_floors.map(move |f| (a, b, f)))
            .filter_map(move |(a, b, next_floor)| {
                let mut new = self.clone();
                let mut mask: Storage = 1 << a;
                if let Some(b) = b {
                    mask |= 1 << b;
                }
                new.floors[new.elevator_at] &= !mask;
                if !Self::is_safe_floor(new.floors[new.elevator_at]) {
                    return None;
                }
                new.floors[next_floor] |= mask;
                if !Self::is_safe_floor(new.floors[next_floor]) {
                    return None;
                }

                new.elevator_at = next_floor;
                Some(new)
            })
    }
}

impl Debug for Facility {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        struct Floor(usize, bool, usize, Storage);
        impl Debug for Floor {
            fn fmt(&self, f: &mut Formatter) -> fmtResult {
                write!(f, "F{} ", self.0 + 1)?;
                if self.1 {
                    write!(f, "E ")?;
                } else {
                    write!(f, ". ")?;
                }
                for i in (0..self.2).rev() {
                    f.write_char(if (self.3 >> (HALF_SIZE + i)) & 1 == 1 {
                        '1'
                    } else {
                        '0'
                    })?;
                }
                f.write_char(' ')?;
                for i in (0..self.2).rev() {
                    f.write_char(if (self.3 >> i) & 1 == 1 { '1' } else { '0' })?;
                }
                Ok(())
            }
        }

        f.debug_list()
            .entries(
                self.floors.iter().enumerate().rev().map(|(idx, floor)| {
                    Floor(idx, idx == self.elevator_at, self.chip_count, *floor)
                }),
            )
            .finish()
    }
}

pub fn parse(s: &str) -> IResult<&str, Vec<Vec<Module>>> {
    use parsers::*;
    map_res(
        fold_many1(
            delimited(
                tag("The "),
                pair(
                    terminated(
                        map_res(alpha1, |s: &str| {
                            Ok(match s {
                                "first" => 1,
                                "second" => 2,
                                "third" => 3,
                                "fourth" => 4,
                                _ => return Err(()),
                            })
                        }),
                        tag(" floor contains "),
                    ),
                    alt((
                        map(tag("nothing relevant"), |_| Vec::new()),
                        separated_list(
                            alt((tag(", and "), tag(" and "), tag(", "))),
                            alt((
                                map(
                                    delimited(tag("a "), alpha1, tag("-compatible microchip")),
                                    Module::Microchip,
                                ),
                                map(
                                    delimited(tag("a "), alpha1, tag(" generator")),
                                    Module::Generator,
                                ),
                            )),
                        ),
                    )),
                ),
                terminated(char('.'), opt(line_ending)),
            ),
            HashMap::new(),
            |mut acc, (floor, modules)| {
                acc.insert(floor, modules);
                acc
            },
        ),
        |mut layout| {
            if layout.len() != 4 {
                return Err(anyhow!("expected 4 floors"));
            }
            let mut res = Vec::with_capacity(4);
            for i in 1..=4 {
                let v = layout
                    .get_mut(&i)
                    .ok_or_else(|| anyhow!("expected floors 1 through 4"))?;
                res.push(Vec::new());
                std::mem::swap(&mut res[i - 1], v);
            }
            Ok(res)
        },
    )(s)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum Module<'s> {
    Generator(&'s str),
    Microchip(&'s str),
}

#[test]
fn day11() -> Result<()> {
    test_part!(parse, pt1, "\
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant." => 11);

    Ok(())
}
