use crate::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Screen(Mat2<bool>);
impl Screen {
    fn new(w: usize, h: usize) -> Self {
        Self(Mat2::new(false, (w, h).into()))
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::Rect(Vec2us { x, y }) => {
                for cx in 0..x {
                    let column = &mut self.0[cx];
                    for cy in 0..y {
                        column[cy] = true;
                    }
                }
            }
            &Instruction::RotateRow { row, amount } => {
                let amount = amount % self.0.width();
                // PERF: Could be optimized to run in O(width)
                //       currently runs in O(width * amount).
                for _ in 0..amount {
                    for x in (0..self.0.width() - 1).rev() {
                        let v = self.0[x][row];
                        self.0[x][row] = self.0[x + 1][row];
                        self.0[x + 1][row] = v;
                    }
                }
            }
            &Instruction::RotateColumn { column, amount } => {
                let column = &mut self.0[column];
                column.rotate_right(amount % column.len());
            }
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for y in 0..self.0.height() {
            for x in 0..self.0.width() {
                if self.0[x][y] {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            if y != self.0.height() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

pub fn pts(input: Vec<Instruction>) -> Result<String> {
    let mut screen = Screen::new(50, 6);
    for instruction in &input {
        screen.apply(instruction);
    }

    Ok(format!(
        "{}\n{}",
        screen.0.data.iter().filter(|x| **x).count(),
        screen
    ))
}

pub fn parse(s: &str) -> IResult<&str, Vec<Instruction>> {
    use parsers::*;
    let instruction = alt((
        map(
            preceded(tag("rect "), pair(usize_str, preceded(tag("x"), usize_str))),
            |(a, b)| Instruction::Rect(Vec2us::new(a, b)),
        ),
        map(
            preceded(
                tag("rotate row y="),
                pair(usize_str, preceded(tag(" by "), usize_str)),
            ),
            |(row, amount)| Instruction::RotateRow { row, amount },
        ),
        map(
            preceded(
                tag("rotate column x="),
                pair(usize_str, preceded(tag(" by "), usize_str)),
            ),
            |(column, amount)| Instruction::RotateColumn { column, amount },
        ),
    ));
    separated_list(line_ending, instruction)(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Rect(Vec2us),
    RotateRow { row: usize, amount: usize },
    RotateColumn { column: usize, amount: usize },
}

#[test]
fn day08() -> Result<()> {
    {
        let mut screen = Screen::new(7, 3);
        screen.apply(&Instruction::Rect((3, 2).into()));
        use std::string::ToString;
        assert_eq!(
            screen.to_string(),
            "\
███░░░░
███░░░░
░░░░░░░".replace("░", " ")
        );
        screen.apply(&Instruction::RotateColumn {
            column: 1,
            amount: 1,
        });
        assert_eq!(
            screen.to_string(),
            "\
█░█░░░░
███░░░░
░█░░░░░".replace("░", " ")
        );
        screen.apply(&Instruction::RotateRow { row: 0, amount: 4 });
        assert_eq!(
            screen.to_string(),
            "\
░░░░█░█
███░░░░
░█░░░░░".replace("░", " ")
        );
        screen.apply(&Instruction::RotateColumn {
            column: 1,
            amount: 1,
        });
        assert_eq!(
            screen.to_string(),
            "\
░█░░█░█
█░█░░░░
░█░░░░░".replace("░", " ")
        );
    }

    Ok(())
}
