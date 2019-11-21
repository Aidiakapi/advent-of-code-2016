use crate::assembunny::*;
use crate::prelude::*;

pub fn pt1(input: Vec<Instruction>) -> Result<i64> {
    let mut prog = Program::new(input)?;
    let mut reg = Registers::default();
    reg[0] = 7;
    prog.run_to_end(&mut reg, |_| {});
    Ok(reg[0])
}

pub fn pt2(input: Vec<Instruction>) -> Result<i64> {
    let mut prog = Program::new(input)?;
    let mut reg = Registers::default();
    reg[0] = 12;
    prog.run_to_end(&mut reg, |_| {});
    Ok(reg[0])
}

pub fn parse(s: &str) -> IResult<&str, Vec<Instruction>> {
    use parsers::*;
    map_res(parse_assembunny, |instrs| {
        for instr in &instrs {
            match instr {
                Instruction::Out(_) => return Err(()),
                _ => {}
            }
        }
        Ok(instrs)
    })(s)
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
