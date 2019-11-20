use crate::prelude::*;

type Vec2 = crate::vec2::Vec2<u16>;

pub fn pt1(input: Vec<Node>) -> Result<usize> {
    // In order to perform this in O(n log n) as opposed to O(n^2)
    // we first create a separate list which just contains all the
    // available space of all the nodes, sorted low-to-high.
    let mut avail_list: Vec<_> = input.iter().map(|node| node.avail).collect();
    avail_list.sort_unstable();

    // The loop is O(n) in which a binary search O(log n) is performed
    // therefore the entire algorithm is O(n log n).
    let mut pair_count = 0;
    for node in input {
        if node.used == 0 {
            continue;
        }
        // For every node a simple binary search will yield the index
        // from which nodes with enough available space start.
        // A custom comparator is used because the index before the
        // nodes with equal size is desired, not an arbitrary node,
        // with the correct amount of space.
        let idx = match avail_list
            .binary_search_by(|avail| avail.cmp(&node.used).then(std::cmp::Ordering::Greater))
        {
            Err(idx) => idx,
            Ok(_) => unreachable!(),
        };
        // Each one of these nodes can form a pair
        pair_count += avail_list.len() - idx;
        // Except if the current node itself had enough space to
        // duplicate its data, in which case we calculated one
        // pair too many, and it has to be removed again.
        if node.avail >= node.used {
            pair_count -= 1;
        }
    }
    Ok(pair_count)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Cell {
    size: u16,
    used: u16,
    dist: u32,
}

fn cell_neighbors(cells: &Mat2<Cell>, pos: Vec2us) -> impl Iterator<Item = Vec2us> {
    let mut neighbors = ArrayVec::<[Vec2us; 4]>::new();

    let size = cells[pos].size;
    if pos.x > 0 && cells[pos.x - 1][pos.y].used <= size {
        neighbors.push(pos - (1, 0).into());
    }
    if pos.y > 0 && cells[pos.x][pos.y - 1].used <= size {
        neighbors.push(pos - (0, 1).into());
    }
    if pos.x < cells.width() - 1 && cells[pos.x + 1][pos.y].used <= size {
        neighbors.push(pos + (1, 0).into());
    }
    if pos.y < cells.height() - 1 && cells[pos.x][pos.y + 1].used <= size {
        neighbors.push(pos + (0, 1).into());
    }

    neighbors.into_iter()
}

pub fn pt2(input: Vec<Node>) -> Result<u32> {
    // Verify the input
    use crate::vec2::AabbIteratorEx;
    let (min, max) = input.iter().map(|node| node.pos).aabb().unwrap();
    if min != 0.into() {
        return Err(anyhow!("expected grid starting at 0,0"));
    }
    let size: Vec2us = (max + 1.into()).into();
    if size.x * size.y != input.len() {
        return Err(anyhow!("expected a grid of nodes"));
    }

    let mut cells = Mat2::new(Cell::default(), size);
    for node in input {
        cells[Vec2us::from(node.pos)] = Cell {
            size: node.avail + node.used,
            used: node.used,
            dist: std::u32::MAX,
        };
    }
    let empty_pos = {
        let mut iter = cells.iter().filter(|(_, cell)| cell.used == 0);
        let empty_pos = match iter.next() {
            Some((pos, _)) => pos,
            None => return Err(anyhow!("expected an empty node")),
        };
        if iter.next().is_some() {
            return Err(anyhow!("found multiple empty nodes"));
        }
        empty_pos
    };

    // Flood fill to differentiate usable nodes from useless ones
    fn flood_fill(cells: &mut Mat2<Cell>, pos: Vec2us, path_length: u32) {
        let cell = &mut cells[pos];
        if cell.dist <= path_length {
            return;
        }
        cell.dist = path_length;

        for neighbor in cell_neighbors(cells, pos) {
            flood_fill(cells, neighbor, path_length + 1);
        }
    }
    flood_fill(&mut cells, empty_pos, 0);

    // Verify that all data in reachable nodes are freely available
    let max_used = cells
        .data
        .iter()
        .filter(|node| node.dist != std::u32::MAX)
        .map(|node| node.used)
        .max()
        .ok_or_else(|| anyhow!("no reachable nodes"))?;
    let min_size = cells
        .data
        .iter()
        .filter(|node| node.dist != std::u32::MAX)
        .map(|node| node.size)
        .min()
        .ok_or_else(|| anyhow!("no reachable nodes"))?;
    if max_used > min_size {
        return Err(anyhow!(
            "reachable nodes cannot have data arbitrarily moved between them"
        ));
    }

    // Check that all the nodes on the top 2 rows are reachable
    if !(0..cells.width())
        .all(|x| cells[x][0].dist != std::u32::MAX && cells[x][1].dist != std::u32::MAX)
    {
        return Err(anyhow!("expect top two rows to be free"));
    }

    // Cost to get the empty cell into the top-right position
    // and the target data just to the left of it.
    let startup_cost = cells[cells.width() - 2][0].dist + 1;
    // To move the data one cell to the left, the empty cell
    // has to move around it: down, left, left, up, right
    let move_cost = 5 * (cells.width() as u32 - 2);

    Ok(startup_cost + move_cost)
}

pub fn parse(s: &str) -> IResult<&str, Vec<Node>> {
    use parsers::*;
    let grid = separated_list(
        line_ending,
        map(
            tuple((
                preceded(tag("/dev/grid/node-x"), u16_str),
                preceded(tag("-y"), u16_str),
                delimited(space1, u16_str, char('T')),
                delimited(space1, u16_str, char('T')),
                delimited(space1, u16_str, char('T')),
                delimited(space1, u16_str, char('%')),
            )),
            |(x, y, _size, used, avail, _use_pct)| Node {
                pos: Vec2::new(x, y),
                used,
                avail,
            },
        ),
    );
    map(
        pair(
            opt(pair(
                opt(preceded(tag("root@ebhq-gridcenter# df -h"), line_ending)),
                tuple((
                    tag("Filesystem"),
                    space1,
                    tag("Size  Used  Avail  Use%"),
                    line_ending,
                )),
            )),
            grid,
        ),
        |(_, g)| g,
    )(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pos: Vec2,
    used: u16,
    avail: u16,
}

#[test]
fn day22() -> Result<()> {
    test_part!(parse, pt2, "\
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%" => 7);

    Ok(())
}
