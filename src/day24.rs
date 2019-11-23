use crate::astar::AStar;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    columns: Vec<u64>,
    height: usize,
    points_of_interest: Vec<Vec2us>,
}

impl Map {
    fn is_wall(&self, pos: Vec2us) -> bool {
        debug_assert!(pos.y < self.height);
        self.columns[pos.x] & (1 << pos.y) == 0
    }
}

fn neighbors(pos: Vec2us) -> [Vec2us; 4] {
    [
        pos - (1, 0).into(),
        pos - (0, 1).into(),
        pos + (1, 0).into(),
        pos + (0, 1).into(),
    ]
}

fn pathfind(
    astar: &mut AStar<Vec2us, usize>,
    from: Vec2us,
    to: Vec2us,
    map: &Map,
) -> Result<usize> {
    let path = astar
        .solve(
            from,
            |&pos| {
                std::array::IntoIter::new(neighbors(pos))
                    .filter(|&new_pos| !map.is_wall(new_pos))
                    .map(|new_pos| (new_pos, 1))
            },
            |&pos| {
                let delta = pos.delta(to);
                delta.x + delta.y
            },
            |&pos| pos == to,
        )
        .ok_or_else(|| anyhow!("no path found"))?;
    Ok(path.last().unwrap().1)
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use fmt::Write;
        for y in 0..self.height {
            for (x, &column) in self.columns.iter().enumerate() {
                f.write_char(
                    if let Some(poi) = self
                        .points_of_interest
                        .iter()
                        .position(|poi| *poi == Vec2us::new(x, y))
                    {
                        (b'0' + poi as u8) as char
                    } else if (column >> y) & 1 == 1 {
                        '.'
                    } else {
                        '#'
                    },
                )?;
            }
            if y + 1 != self.height {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pts(pub usize, pub usize);
impl Display for Pts {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "pt1 {}\npt2 {}", self.0, self.1)
    }
}

// This day is a variation of the Travelling Salesman Problem, which is a
// well known problem in computer science. There is currently no known
// algorithm to find an exact solution in a better time than O(n^2*2^n).
// However, since the amount of places to evaluate is only 8 in the input
// I've chosen to perform a brute-force approach, with the one optimization
// being that I precompute all the path lengths between the places.
pub fn pts(map: Map) -> Result<Pts> {
    let mut astar = AStar::new();

    // First pre-compute the length to go from any of the points of interest
    // to any of the other points of interest. This is O(n^2).
    let mut path_lengths: Vec<Vec<usize>> = Vec::with_capacity(map.points_of_interest.len());
    for i in 0..map.points_of_interest.len() {
        let mut from_this_point = Vec::with_capacity(map.points_of_interest.len());
        for j in 0..i {
            from_this_point.push(path_lengths[j][i]);
        }
        from_this_point.push(0);
        for j in i + 1..map.points_of_interest.len() {
            from_this_point.push(pathfind(
                &mut astar,
                map.points_of_interest[i],
                map.points_of_interest[j],
                &map,
            )?);
        }
        path_lengths.push(from_this_point);
    }

    // Then permute the points of interest, and calculate the sum of the path sections
    // for each permutation. This is O(N!).
    let mut min_path_len = Pts(std::usize::MAX, std::usize::MAX);
    for permutation in
        (1..map.points_of_interest.len()).permutations(map.points_of_interest.len() - 1)
    {
        let mut from = 0;
        let mut total_len = 0;
        for to in permutation {
            total_len += path_lengths[from][to];
            from = to;
        }
        min_path_len.0 = min_path_len.0.min(total_len);
        total_len += path_lengths[from][0];
        min_path_len.1 = min_path_len.1.min(total_len);
    }

    Ok(min_path_len)
}

pub fn parse(s: &str) -> IResult<&str, Map> {
    use parsers::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Cell {
        Wall,
        Open,
        Nr(u8),
    }
    let cell = alt((
        map(
            one_of("#."),
            |c: char| if c == '#' { Cell::Wall } else { Cell::Open },
        ),
        map(one_of("0123456789"), |c: char| Cell::Nr((c as u8) - b'0')),
    ));

    let row = many1(cell);
    let grid = separated_list(line_ending, row);
    map_res(grid, |rows: Vec<Vec<Cell>>| {
        let height = rows.len();
        if height < 3 || height > 64 {
            return Err(anyhow!("map height must be in 3..=64 but is {}", height));
        }
        let width = rows[0].len();
        if width < 3 {
            return Err(anyhow!("map width must be > 3 but is {}", width));
        }
        if !rows.iter().all(|row| row.len() == width) {
            return Err(anyhow!("input is not rectangular grid"));
        }
        let mut map = Map {
            columns: vec![0; width],
            points_of_interest: Vec::new(),
            height,
        };

        for (y, row) in rows.into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                match cell {
                    Cell::Wall => {}
                    Cell::Open => {
                        map.columns[x] |= 1 << y;
                    }
                    Cell::Nr(n) => {
                        let n = n as usize;
                        map.columns[x] |= 1 << y;
                        if map.points_of_interest.len() < n + 1 {
                            map.points_of_interest
                                .resize(n + 1, (std::usize::MAX).into());
                        }
                        map.points_of_interest[n] = (x, y).into();
                    }
                }
            }
        }

        if map
            .points_of_interest
            .iter()
            .any(|v| *v == (std::usize::MAX).into())
        {
            return Err(anyhow!("holes in points of interest"));
        }
        if map.points_of_interest.len() < 2 {
            return Err(anyhow!("expected at least 2 points of interest"));
        }
        let check_mask_top_bottom = 1 | (1 << (height - 1));
        if map.columns[0] != 0
            || map.columns[width - 1] != 0
            || map.columns[1..width - 1]
                .iter()
                .any(|column| column & check_mask_top_bottom != 0)
        {
            return Err(anyhow!("map must be surrounded by a border of walls"));
        }

        Ok(map)
    })(s)
}

#[test]
fn day24() -> Result<()> {
    use std::string::ToString;
    const EXAMPLE: &'static str = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
    let example = parse(EXAMPLE).unwrap().1;
    assert_eq!(EXAMPLE, &example.to_string());

    test_part!(|input| pts(input).map(|Pts(pt1, _)| pt1), example.clone() => 14);

    Ok(())
}
