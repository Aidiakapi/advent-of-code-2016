use crate::prelude::*;

pub fn pt1(program: Vec<Instruction>) -> Result<i64> {
    let mut regs = Registers::default();
    regs.execute_program(&program);
    Ok(regs.0[0])
}

pub fn pt2(program: Vec<Instruction>) -> Result<i64> {
    let mut regs = Registers::default();
    regs.0[2] = 1;
    regs.execute_program(&program);
    Ok(regs.0[0])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Register(usize),
    Literal(i64),
}

impl Value {
    fn eval(&self, regs: &Registers) -> i64 {
        match self {
            &Value::Register(idx) => regs.0[idx],
            &Value::Literal(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Registers([i64; 4]);

impl Registers {
    fn execute_program(&mut self, program: &Vec<Instruction>) {
        let mut ip = 0isize;
        while ip >= 0 && (ip as usize) < program.len() {
            let instruction = &program[ip as usize];
            match instruction {
                &Instruction::Copy(value, target) => {
                    let val = value.eval(&self);
                    self.0[target] = val;
                }
                &Instruction::Increment(reg) => self.0[reg] += 1,
                &Instruction::Decrement(reg) => self.0[reg] -= 1,
                &Instruction::JumpIfNotZero(cond, target) => {
                    let val = cond.eval(&self);
                    if val != 0 {
                        ip += target;
                        continue;
                    }
                }
            }
            ip += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Copy(Value, usize),
    Increment(usize),
    Decrement(usize),
    JumpIfNotZero(Value, isize),
}

pub fn parse(s: &str) -> IResult<&str, Vec<Instruction>> {
    use parsers::*;
    fn parse_register(s: &str) -> IResult<&str, usize> {
        map_res(anychar, |c| {
            if c >= 'a' && c <= 'z' {
                Ok((c as u8 - b'a') as usize)
            } else {
                Err(())
            }
        })(s)
    }
    fn parse_value(s: &str) -> IResult<&str, Value> {
        alt((
            map(parse_register, Value::Register),
            map(i64_str, Value::Literal),
        ))(s)
    }
    #[rustfmt::skip]
    fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
        alt((
            map(preceded(tag("cpy "), pair(parse_value, preceded(char(' '), parse_register))), |(a, b)| Instruction::Copy(a, b)),
            map(preceded(tag("inc "), parse_register), Instruction::Increment),
            map(preceded(tag("dec "), parse_register), Instruction::Decrement),
            map(preceded(tag("jnz "), pair(parse_value, preceded(char(' '), isize_str))), |(a, b)| Instruction::JumpIfNotZero(a, b)),
        ))(s)
    }

    separated_list(line_ending, parse_instruction)(s)
}

#[test]
pub fn day12() -> Result<()> {
    const EXAMPLE: &'static str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
    let example: Vec<Instruction> = vec![
        Instruction::Copy(Value::Literal(41), 0),
        Instruction::Increment(0),
        Instruction::Increment(0),
        Instruction::Decrement(0),
        Instruction::JumpIfNotZero(Value::Register(0), 2),
        Instruction::Decrement(0),
    ];
    test_parse!(parse, EXAMPLE => example);

    test_part!(pt1, example.clone() => 42);

    Ok(())
}
