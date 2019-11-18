use crate::prelude::*;
use Tile::*;

fn get_tile(nw: Tile, n: Tile, ne: Tile) -> Tile {
    match (nw, n, ne) {
        (Trap, _, Trap) => Safe,
        (Trap, _, _) => Trap,
        (_, _, Trap) => Trap,
        _ => Safe,
    }
}

fn solve(mut row: Vec<Tile>, height: usize) -> Result<usize> {
    let width = row.len();
    if width < 2 {
        return Err(anyhow!("expected floor width to be at least 2"));
    }
    let mut new_row = Vec::with_capacity(width);

    let mut safe_tile_count = row.iter().filter(|tile| **tile == Safe).count();
    for _ in 1..height {
        new_row.clear();
        new_row.push(get_tile(Safe, row[0], row[1]));
        for x in 1..width - 1 {
            new_row.push(get_tile(row[x - 1], row[x], row[x + 1]));
        }
        new_row.push(get_tile(row[width - 2], row[width - 1], Safe));
        safe_tile_count += new_row.iter().filter(|tile| **tile == Safe).count();
        assert_eq!(new_row.len(), width);
        assert_eq!(row.len(), width);
        std::mem::swap(&mut row, &mut new_row);
    }
    Ok(safe_tile_count)
}

pub fn pt1(input: Vec<Tile>) -> Result<usize> {
    solve(input, 40)
}

pub fn pt2(input: Vec<Tile>) -> Result<usize> {
    solve(input, 400000)
}

pub fn parse(s: &str) -> IResult<&str, Vec<Tile>> {
    use parsers::*;
    many1(alt((map(char('.'), |_| Safe), map(char('^'), |_| Trap))))(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Safe,
    Trap,
}

#[test]
fn day18() -> Result<()> {
    test_part!(parse, |i| solve(i, 10), ".^^.^.^^^^" => 38);

    Ok(())
}
