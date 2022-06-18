use crate::prelude::*;

trait FacilityBounds = Clone + Eq + Ord + std::hash::Hash;

fn solve<const N: usize>(input: Vec<Vec<Module>>) -> Result<usize>
where
    [Element; N]: Default,
    [bool; N]: Default,
{
    use std::{collections::VecDeque, rc::Rc};

    let facility: Facility<[Element; N]> = Facility::from_input(input)?;
    struct TrackedFacility<E: FacilityBounds> {
        parent: Option<Rc<Self>>,
        current: Facility<E>,
    }
    impl<const N: usize> TrackedFacility<[Element; N]>
    where
        [Element; N]: Default,
        [bool; N]: Default,
    {
        pub fn depth(&self) -> usize {
            match self.parent.as_ref() {
                Some(parent) => parent.depth() + 1,
                None => 0,
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert(facility.clone());
    let mut bfs = VecDeque::new();
    bfs.push_back(Rc::new(TrackedFacility {
        parent: None,
        current: facility,
    }));

    while let Some(tracked) = bfs.pop_front() {
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
            bfs.push_back(child);
        }
    }

    Err(anyhow!("no solution found"))
}

pub fn pt1(input: Vec<Vec<Module>>) -> Result<usize> {
    solve::<5>(input)
}

pub fn pt2(mut input: Vec<Vec<Module>>) -> Result<usize> {
    input[0].push(Module::Generator("elerium"));
    input[0].push(Module::Microchip("elerium"));
    input[0].push(Module::Generator("dilithium"));
    input[0].push(Module::Microchip("dilithium"));
    solve::<7>(input)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[repr(packed)]
struct Element(u8);

impl Element {
    #[inline(always)]
    fn generator(&self) -> u8 {
        self.0 >> 4
    }

    #[inline(always)]
    fn microchip(&self) -> u8 {
        self.0 & 0x0f
    }

    #[inline(always)]
    fn move_generator(&mut self, new_floor: u8) {
        debug_assert!(new_floor & 0xf0 == 0);
        self.0 = (self.0 & 0x0f) | (new_floor << 4);
    }

    #[inline(always)]
    fn move_microchip(&mut self, new_floor: u8) {
        debug_assert!(new_floor & 0xf0 == 0);
        self.0 = (self.0 & 0xf0) | new_floor;
    }
}

const FLOOR_COUNT: u8 = 4;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Facility<E: FacilityBounds> {
    elements: E,
    elevator_position: u8,
}

impl<const N: usize> Facility<[Element; N]>
where
    [Element; N]: Default,
    [bool; N]: Default,
{
    fn from_input(input: Vec<Vec<Module>>) -> Result<Self> {
        assert!(N <= 8);
        if input.len() != FLOOR_COUNT as usize {
            return Err(anyhow!("invalid floor count"));
        }

        let mut element_map = HashMap::new();
        for (floor_idx, floor) in input.into_iter().rev().enumerate() {
            for module in floor {
                let elem_map_len = element_map.len();
                let value = element_map
                    .entry(module.name())
                    .or_insert((elem_map_len, None, None));
                let opt = match module {
                    Module::Generator(_) => &mut value.1,
                    Module::Microchip(_) => &mut value.2,
                };
                if let Some(previous_floor) = opt {
                    return Err(anyhow!(
                        "duplicate module {:?} (floors {} and {})",
                        module,
                        FLOOR_COUNT as usize - floor_idx,
                        FLOOR_COUNT as usize - *previous_floor
                    ));
                }
                *opt = Some(floor_idx);
            }
        }
        if element_map.len() != N {
            return Err(anyhow!(
                "expected {} elements, got {}",
                N,
                element_map.len()
            ));
        }
        if !element_map
            .values()
            .all(|(_, a, b)| a.is_some() && b.is_some())
        {
            return Err(anyhow!(
                "modules must be matching generator & microchip pairs"
            ));
        }
        let mut elements: [Element; N] = Default::default();
        for (_, (elem_idx, generator_floor, microchip_floor)) in element_map {
            elements[elem_idx].move_generator(generator_floor.unwrap() as u8);
            elements[elem_idx].move_microchip(microchip_floor.unwrap() as u8);
        }

        let mut facility = Facility {
            elements,
            elevator_position: FLOOR_COUNT - 1,
        };
        facility.normalize();
        if !facility.is_safe_configuration() {
            return Err(anyhow!("unsafe starting conditions"));
        }
        Ok(facility)
    }

    fn normalize(&mut self) {
        self.elements.sort_unstable()
    }

    fn is_solved(&self) -> bool {
        self.elements.iter().all(|e| e.0 == 0)
    }

    fn is_safe_configuration(&self) -> bool {
        let mut has_generator: [bool; FLOOR_COUNT as usize] = Default::default();
        let mut has_unpaired_microchip: [bool; FLOOR_COUNT as usize] = Default::default();
        for elem in &self.elements {
            let generator_floor = elem.generator();
            let microchip_floor = elem.microchip();
            has_generator[elem.generator() as usize] = true;
            if generator_floor != microchip_floor {
                has_unpaired_microchip[microchip_floor as usize] = true;
            }
        }
        (0..FLOOR_COUNT as usize).all(|i| !has_generator[i] || !has_unpaired_microchip[i])
    }

    fn next_configurations<'s>(&'s self) -> impl Iterator<Item = Facility<[Element; N]>> + 's {
        use std::iter::ExactSizeIterator;
        #[derive(Clone, Copy)]
        struct NextFloors {
            a: u8,
            b: u8,
            l: u8,
            c: u8,
        }
        impl Iterator for NextFloors {
            type Item = u8;
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
            if self.elevator_position == FLOOR_COUNT - 1
        {
            if self.elevator_position == 0 {
                NextFloors { a: 0, b: 0, l: 0, c: 0 }
            } else {
                NextFloors { a: self.elevator_position - 1, b: 0, l: 1, c: 0 }
            }
        } else {
            if self.elevator_position == 0 {
                NextFloors { a: self.elevator_position + 1, b: 0, l: 1, c: 0 }
            } else {
                NextFloors { a: self.elevator_position - 1, b: self.elevator_position + 1, l: 2, c: 0 }
            }
        };

        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Moveable {
            Microchip(u8),
            Generator(u8),
        }
        let elevator_position = self.elevator_position;
        let moveable_items = self
            .elements
            .iter()
            .enumerate()
            .flat_map(move |(idx, elem)| {
                let idx = idx as u8;
                let mut array: ArrayVec<Moveable, 2> = ArrayVec::new();
                unsafe {
                    if elem.generator() == elevator_position {
                        array.push_unchecked(Moveable::Generator(idx));
                    }
                    if elem.microchip() == elevator_position {
                        array.push_unchecked(Moveable::Microchip(idx));
                    }
                }
                array.into_iter()
            });

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
                new.elevator_position = next_floor;

                #[rustfmt::skip] match a {
                    Moveable::Generator(idx) => new.elements[idx as usize].move_generator(next_floor),
                    Moveable::Microchip(idx) => new.elements[idx as usize].move_microchip(next_floor),
                }

                if let Some(b) = b {
                    #[rustfmt::skip] match b {
                        Moveable::Generator(idx) => new.elements[idx as usize].move_generator(next_floor),
                        Moveable::Microchip(idx) => new.elements[idx as usize].move_microchip(next_floor),
                    }
                }

                if new.is_safe_configuration() {
                    new.normalize();
                    Some(new)
                } else {
                    None
                }
            })
    }
}

impl Debug for Element {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "G({})xM({})", self.generator(), self.microchip())
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
                        separated_list1(
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
            || HashMap::new(),
            |mut acc, (floor, modules)| {
                acc.insert(floor, modules);
                acc
            },
        ),
        |mut layout| {
            if layout.len() != FLOOR_COUNT as usize {
                return Err(anyhow!("expected {} floors", FLOOR_COUNT));
            }
            let mut res = Vec::with_capacity(FLOOR_COUNT as usize);
            for i in 1..=FLOOR_COUNT as usize {
                let v = layout
                    .get_mut(&i)
                    .ok_or_else(|| anyhow!("expected floors 1 through {}", FLOOR_COUNT))?;
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

impl<'s> Module<'s> {
    fn name(&self) -> &'s str {
        match self {
            Module::Generator(s) => s,
            Module::Microchip(s) => s,
        }
    }
}

#[test]
fn day11() -> Result<()> {
    fn solve_2(input: Vec<Vec<Module>>) -> Result<usize> {
        solve::<2>(input)
    }
    test_part!(parse, solve_2, "\
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant." => 11);

    Ok(())
}
