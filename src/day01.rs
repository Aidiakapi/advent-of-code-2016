use crate::prelude::*;

pub fn pt1(input: Vec<Action>) -> Result<i32> {
    let (_, dx, dy) =
        input
            .into_iter()
            .fold((Facing::North, 0, 0), |(facing, xpos, ypos), action| {
                let facing = facing.rotate(action.rotation);
                let (dx, dy) = facing.vec();
                (facing, xpos + dx * action.amount, ypos + dy * action.amount)
            });
    Ok(dx.abs() + dy.abs())
}

pub fn pt2(input: Vec<Action>) -> Result<i32> {
    let mut visited = HashSet::new();
    let mut facing = Facing::North;
    let mut xpos: i32 = 0;
    let mut ypos: i32 = 0;
    visited.insert((0, 0));
    for action in input {
        facing = facing.rotate(action.rotation);
        for _ in 0..action.amount {
            let (dx, dy) = facing.vec();
            xpos += dx;
            ypos += dy;
            if !visited.insert((xpos, ypos)) {
                return Ok(xpos.abs() + ypos.abs());
            }
        }
    }
    Err(anyhow!("no duplicate position found"))
}

pub fn parse(s: &str) -> nom::IResult<&str, Vec<Action>> {
    use parsers::*;
    let action = map(
        pair(
            map_res(one_of("LR"), |c| match c {
                'L' => Ok(Rotate::Left),
                'R' => Ok(Rotate::Right),
                _ => Err(()),
            }),
            u32_str,
        ),
        |(rotation, amount)| Action {
            rotation,
            amount: amount as i32,
        },
    );

    separated_list1(tag(", "), action)(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Action {
    rotation: Rotate,
    amount: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotate {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    North,
    East,
    South,
    West,
}
impl Facing {
    fn rotate(self, rotation: Rotate) -> Facing {
        match rotation {
            Rotate::Left => self.rotate_left(),
            Rotate::Right => self.rotate_right(),
        }
    }
    fn rotate_right(self) -> Facing {
        use Facing::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn rotate_left(self) -> Facing {
        use Facing::*;
        match self {
            East => North,
            South => East,
            West => South,
            North => West,
        }
    }

    fn vec(self) -> (i32, i32) {
        use Facing::*;
        match self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }
}

#[test]
fn day01() -> Result<()> {
    test_parse!(parse, "R8, R4, R4, R8" => vec![
        Action { rotation: Rotate::Right, amount: 8 },
        Action { rotation: Rotate::Right, amount: 4 },
        Action { rotation: Rotate::Right, amount: 4 },
        Action { rotation: Rotate::Right, amount: 8 },
    ]);

    test_part!(parse, pt1,
        "R2, L3" => 5,
        "R2, R2, R2" => 2,
        "R5, L5, R5, R3" => 12,
    );

    test_part!(parse, pt2, "R8, R4, R4, R8" => 4);

    test_part!(pt2, vec![
        Action { rotation: Rotate::Right, amount: 8 },
        Action { rotation: Rotate::Right, amount: 4 },
        Action { rotation: Rotate::Right, amount: 4 },
        Action { rotation: Rotate::Right, amount: 8 },
    ] => 4);

    Ok(())
}
