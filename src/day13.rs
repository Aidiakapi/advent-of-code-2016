use crate::prelude::*;

pub fn pt1(input: usize) -> Result<usize> {
    let is_free = is_free_fn(input);
    let mut astar = crate::astar::AStar::new();
    let path = astar
        .solve(
            Vec2us::new(1, 1),
            move |pos: &Vec2us| Neighbors(*pos, 0).filter(is_free).zip(repeat(1)),
            |pos: &Vec2us| ((pos.x as isize) - 31).abs() + ((pos.y as isize) - 39).abs(),
            |pos: &Vec2us| pos.x == 31 && pos.y == 39,
        )
        .ok_or_else(|| anyhow!("no path found"))?;
    Ok(path.len() - 1)
}

pub fn pt2(input: usize) -> Result<usize> {
    let is_free = is_free_fn(input);
    let mut map = HashMap::new();
    fn add_to_map<F>(
        map: &mut HashMap<Vec2us, usize>,
        pos: Vec2us,
        dist_from_start: usize,
        is_free: F,
    ) where
        F: Fn(&Vec2us) -> bool + Clone + Copy,
    {
        if dist_from_start == 50 {
            return;
        }
        for neighbor in Neighbors(pos, 0).filter(is_free) {
            use std::collections::hash_map::Entry;
            match map.entry(neighbor) {
                Entry::Occupied(mut entry) if *entry.get() > dist_from_start => {
                    *entry.get_mut() = dist_from_start + 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(dist_from_start + 1);
                }
                Entry::Occupied(_) => continue,
            }
            add_to_map(map, neighbor, dist_from_start + 1, is_free);
        }
    }
    map.insert(Vec2us::new(1, 1), 0);
    add_to_map(&mut map, Vec2us::new(1, 1), 0, is_free);
    Ok(map.len())
}

fn is_free_fn(designer_nr: usize) -> impl Fn(&Vec2us) -> bool + Clone + Copy {
    move |p| {
        (p.x * p.x + 3 * p.x + 2 * p.x * p.y + p.y + p.y * p.y + designer_nr).count_ones() % 2 == 0
    }
}

pub fn parse(s: &str) -> IResult<&str, usize> {
    use parsers::*;
    usize_str(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Neighbors(Vec2us, usize);
impl Iterator for Neighbors {
    type Item = Vec2us;
    fn next(&mut self) -> Option<Self::Item> {
        match self.1 {
            0 => {
                self.1 = 1;
                Some(Vec2us::new(self.0.x + 1, self.0.y))
            }
            1 => {
                self.1 = if self.0.x == 0 {
                    if self.0.y == 0 {
                        4
                    } else {
                        3
                    }
                } else {
                    2
                };
                Some(Vec2us::new(self.0.x, self.0.y + 1))
            }
            2 => {
                self.1 = if self.0.y == 0 { 4 } else { 3 };
                Some(Vec2us::new(self.0.x - 1, self.0.y))
            }
            3 => {
                self.1 = 4;
                Some(Vec2us::new(self.0.x, self.0.y - 1))
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl std::iter::ExactSizeIterator for Neighbors {
    fn len(&self) -> usize {
        4 - if self.0.x == 0 { 1 } else { 0 } - if self.0.y == 0 { 1 } else { 0 } - self.1
    }
}

#[test]
fn day13() -> Result<()> {
    let is_free = is_free_fn(10);
    let mut board = String::with_capacity(7 * 11);
    for y in 0..7 {
        for x in 0..10 {
            board.push(if is_free(&(x, y).into()) { '.' } else { '#' });
        }
        board.push('\n');
    }

    assert_eq!(
        &board,
        "\
.#.####.##
..#..#...#
#....##...
###.#.###.
.##..#..#.
..##....#.
#...##.###
"
    );

    let mut astar = crate::astar::AStar::new();
    let path = astar
        .solve(
            Vec2us::new(1, 1),
            move |pos: &Vec2us| Neighbors(*pos, 0).filter(is_free).zip(repeat(1)),
            |pos: &Vec2us| ((pos.x as isize) - 7).abs() + ((pos.y as isize) - 4).abs(),
            |pos: &Vec2us| pos.x == 7 && pos.y == 4,
        )
        .expect("no path found");
    assert_eq!(path.len(), 12);
    assert_eq!(path[11].0, Vec2us::new(7, 4));
    assert_eq!(path[11].1, 11);

    Ok(())
}
