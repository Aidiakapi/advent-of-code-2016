use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// 128 bits in total
// 4 bits used for position
// 6 bits for path length (0-63)
// 118 bits for path data, 2 bits per node, or 59 nodes
struct Node(u64, u64);
impl Node {
    const MAX_LENGTH: usize = 64 - (Node::SHIFT_PATH as usize) / 2;
    const META_LENGTH: usize = Node::MAX_LENGTH - 32;

    const MASK_POS: u64 = 0b11;
    const MASK_LEN: u64 = 0b111111;
    const MASK_PATH: u64 = 0b11;
    const SHIFT_X: u64 = 0;
    const SHIFT_Y: u64 = 2;
    const SHIFT_LEN: u64 = 4;
    const SHIFT_PATH: u64 = 10;
    const DIRECTIONS: [Direction; 4] = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];

    #[inline]
    fn new() -> Node {
        Node(0, 0)
    }

    #[inline]
    fn x(&self) -> usize {
        ((self.1 >> Node::SHIFT_X) & Node::MASK_POS) as usize
    }
    #[inline]
    fn y(&self) -> usize {
        ((self.1 >> Node::SHIFT_Y) & Node::MASK_POS) as usize
    }
    #[inline]
    fn len(&self) -> usize {
        ((self.1 >> Node::SHIFT_LEN) & Node::MASK_LEN) as usize
    }

    fn try_move_into(&self, d: Direction) -> Option<Node> {
        let len = self.len();
        assert!(len < Node::MAX_LENGTH);
        let x = self.x();
        let y = self.y();
        let (nx, ny, direction_idx) = match d {
            Direction::Right if x < 3 => (x + 1, y, 0),
            Direction::Down if y < 3 => (x, y + 1, 1),
            Direction::Left if x > 0 => (x - 1, y, 2),
            Direction::Up if y > 0 => (x, y - 1, 3),
            _ => return None,
        };
        let nx = nx as u64;
        let ny = ny as u64;
        let mut new_node = self.clone();
        const CLEAR_MASK: u64 = !((Node::MASK_LEN << Node::SHIFT_LEN)
            | (Node::MASK_POS << Node::SHIFT_X)
            | (Node::MASK_POS << Node::SHIFT_Y));
        new_node.1 &= CLEAR_MASK;
        new_node.1 |=
            (nx << Node::SHIFT_X) | (ny << Node::SHIFT_Y) | (((len + 1) as u64) << Node::SHIFT_LEN);
        if len < Node::META_LENGTH {
            new_node.1 |= direction_idx << (Node::SHIFT_PATH as usize + len * 2);
        } else {
            new_node.0 |= direction_idx << ((len - Node::META_LENGTH) * 2);
        }
        Some(new_node)
    }
}

impl std::ops::Index<usize> for Node {
    type Output = Direction;
    fn index(&self, idx: usize) -> &Self::Output {
        debug_assert!(idx < self.len());
        let val = if idx < Node::META_LENGTH {
            (self.1 >> (Node::SHIFT_PATH as usize + idx * 2)) & Node::MASK_PATH
        } else {
            (self.0 >> ((idx - Node::META_LENGTH) * 2)) & Node::MASK_PATH
        } as usize;
        &Node::DIRECTIONS[val]
    }
}

impl IntoIterator for Node {
    type IntoIter = PathIter;
    type Item = Direction;
    fn into_iter(self) -> PathIter {
        PathIter {
            node: self,
            cursor: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PathIter {
    node: Node,
    cursor: usize,
}
impl Iterator for PathIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.node.len() {
            None
        } else {
            let res = Some(self.node[self.cursor]);
            self.cursor += 1;
            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl ExactSizeIterator for PathIter {
    fn len(&self) -> usize {
        self.node.len()
    }
}

pub fn pt1(input: Vec<u8>) -> Result<String> {
    let mut astar = crate::astar::AStar::new();

    let base_len = input.len();
    let mut buf = Vec::new();
    buf.extend(input);

    fn write_path_to_buf(buf: &mut Vec<u8>, n: &Node) {
        buf.extend(n.into_iter().map(|dir| match dir {
            Direction::Right => b'R',
            Direction::Down => b'D',
            Direction::Left => b'L',
            Direction::Up => b'U',
        }));
    }

    let path = astar.solve(
        Node::new(),
        |&n| {
            buf.truncate(base_len);
            write_path_to_buf(&mut buf, &n);

            let digest = md5::compute(&buf);
            let (updown, leftright) = (digest[0], digest[1]);
            let mut dirs: ArrayVec<Direction, 4> = ArrayVec::new();
            if (updown >> 4) > 10 {
                dirs.push(Direction::Up);
            }
            if (updown & 0xf) > 10 {
                dirs.push(Direction::Down);
            }
            if (leftright >> 4) > 10 {
                dirs.push(Direction::Left);
            }
            if (leftright & 0xf) > 10 {
                dirs.push(Direction::Right);
            }

            dirs.into_iter()
                .filter_map(move |dir| n.try_move_into(dir).map(|node| (node, 1)))
        },
        |n| (3 - n.x()) + (3 - n.y()),
        |n| n.x() == 3 && n.y() == 3,
    );

    if let Some(path) = path {
        let node = path.last().unwrap().0;
        Ok(node
            .into_iter()
            .map(|dir| match dir {
                Direction::Right => 'R',
                Direction::Down => 'D',
                Direction::Left => 'L',
                Direction::Up => 'U',
            })
            .collect())
    } else {
        Err(anyhow!("no possible path to exit"))
    }
}

pub fn pt2(input: Vec<u8>) -> Result<usize> {
    let base_len = input.len();
    let mut longest = base_len;
    let mut buf = input;

    fn dfs(longest: &mut usize, buf: &mut Vec<u8>, x: usize, y: usize) {
        // Exit condition is if we've reached the destination
        if x == 3 && y == 3 {
            *longest = (*longest).max(buf.len());
            return;
        }

        // Recurse to all valid neighbors
        let digest = md5::compute(&buf);
        let (updown, leftright) = (digest[0], digest[1]);
        if (updown >> 4) > 10 && y > 0 {
            buf.push(b'U');
            dfs(longest, buf, x, y - 1);
            buf.pop();
        }
        if (updown & 0xf) > 10 && y < 3 {
            buf.push(b'D');
            dfs(longest, buf, x, y + 1);
            buf.pop();
        }
        if (leftright >> 4) > 10 && x > 0 {
            buf.push(b'L');
            dfs(longest, buf, x - 1, y);
            buf.pop();
        }
        if (leftright & 0xf) > 10 && x < 3 {
            buf.push(b'R');
            dfs(longest, buf, x + 1, y);
            buf.pop();
        }
    }
    dfs(&mut longest, &mut buf, 0, 0);

    Ok(longest - base_len)
}

pub fn parse(s: &str) -> IResult<&str, Vec<u8>> {
    use parsers::*;
    map(alpha1, |c: &str| Vec::from(c.as_bytes()))(s)
}

#[test]
fn day17() -> Result<()> {
    test_part!(parse, pt1,
        "ihgpwlah" => "DDRRRD",
        "kglvqrro" => "DDUDRLRRUDRD",
        "ulqzkmiv" => "DRURDRUDDLLDLUURRDULRLDUUDDDRR",
    );
    test_part!(parse, pt2,
        "ihgpwlah" => 370,
        "kglvqrro" => 492,
        "ulqzkmiv" => 830,
    );

    Ok(())
}
