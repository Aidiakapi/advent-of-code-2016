use crate::assembunny::*;
use crate::prelude::*;

pub fn pt1(input: Vec<Instruction>) -> Result<i64> {
    let mut prog = Program::new(input)?;
    let mut reg = Registers::default();
    reg[0] = 7;
    prog.run_to_end(&mut reg);
    Ok(reg[0])
}

pub fn pt2(input: Vec<Instruction>) -> Result<i64> {
    let mut prog = Program::new(input)?;
    let mut reg = Registers::default();
    reg[0] = 12;
    prog.run_to_end(&mut reg);
    Ok(reg[0])
}

pub fn parse(s: &str) -> IResult<&str, Vec<Instruction>> {
    use parsers::*;
    fn instr_1op<I>(name: &'static str, instr: I) -> impl Fn(&str) -> IResult<&str, Instruction>
    where
        I: Fn(usize) -> Instruction,
    {
        move |s: &str| {
            map(
                preceded(preceded(tag(name), char(' ')), parse_register),
                &instr,
            )(s)
        }
    }
    fn instr_2op<I>(name: &'static str, instr: I) -> impl Fn(&str) -> IResult<&str, Instruction>
    where
        I: Fn(Value, Value) -> Instruction,
    {
        move |s: &str| {
            map(
                preceded(
                    preceded(tag(name), char(' ')),
                    pair(parse_value, preceded(char(' '), parse_value)),
                ),
                |(a, b)| instr(a, b),
            )(s)
        }
    }

    let instruction = alt((
        instr_1op("tgl", Instruction::Toggle),
        instr_1op("inc", Instruction::Increment),
        instr_1op("dec", Instruction::Decrement),
        instr_2op("cpy", Instruction::Copy),
        instr_2op("jnz", Instruction::JumpIfNotZero),
    ));

    separated_list(line_ending, instruction)(s)
}

#[test]
fn day23() -> Result<()> {
    test_part!(parse, pt1, "\
cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a" => 3);

    Ok(())
}
