use crate::assembunny::*;
use crate::prelude::*;

pub fn pt1(program: Vec<Instruction>) -> Result<i64> {
    let mut regs = Registers::default();
    let mut program = Program::new(program)?;
    program.run_to_end(&mut regs);
    Ok(regs[0])
}

pub fn pt2(program: Vec<Instruction>) -> Result<i64> {
    let mut regs = Registers::default();
    regs[2] = 1;
    let mut program = Program::new(program)?;
    program.run_to_end(&mut regs);
    Ok(regs[0])
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
            map(i64_str, Value::Constant),
        ))(s)
    }
    #[rustfmt::skip]
    fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
        alt((
            map(preceded(tag("cpy "), pair(parse_value, preceded(char(' '), parse_register))), |(a, b)| Instruction::Copy(a, Value::Register(b))),
            map(preceded(tag("inc "), parse_register), Instruction::Increment),
            map(preceded(tag("dec "), parse_register), Instruction::Decrement),
            map(preceded(tag("jnz "), pair(parse_value, preceded(char(' '), i64_str))), |(a, b)| Instruction::JumpIfNotZero(a, Value::Constant(b))),
        ))(s)
    }

    separated_list(line_ending, parse_instruction)(s)
}

#[test]
fn day12() -> Result<()> {
    const EXAMPLE: &'static str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
    let example: Vec<Instruction> = vec![
        Instruction::Copy(Value::Constant(41), Value::Register(0)),
        Instruction::Increment(0),
        Instruction::Increment(0),
        Instruction::Decrement(0),
        Instruction::JumpIfNotZero(Value::Register(0), Value::Constant(2)),
        Instruction::Decrement(0),
    ];
    test_parse!(parse, EXAMPLE => example);

    test_part!(pt1, example.clone() => 42);

    Ok(())
}
