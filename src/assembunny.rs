use anyhow::anyhow;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub instruction_ptr: i64,
}

pub const REGISTERY_SIZE: usize = 4;
pub type Registers = [i64; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Constant(i64),
    Register(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Toggle(usize),
    Increment(usize),
    Decrement(usize),
    Out(Value),
    Copy(Value, Value),
    JumpIfNotZero(Value, Value),
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> crate::prelude::Result<Self> {
        for instr in &instructions {
            match instr {
                Instruction::Toggle(reg)
                | Instruction::Increment(reg)
                | Instruction::Decrement(reg)
                | Instruction::Copy(Value::Register(reg), Value::Constant(_))
                | Instruction::JumpIfNotZero(Value::Register(reg), Value::Constant(_))
                | Instruction::Copy(Value::Constant(_), Value::Register(reg))
                | Instruction::JumpIfNotZero(Value::Constant(_), Value::Register(reg))
                    if *reg > REGISTERY_SIZE =>
                {
                    return Err(anyhow!("invalid register in instruction {:?}", instr));
                }
                Instruction::Copy(Value::Register(a), Value::Register(b))
                | Instruction::JumpIfNotZero(Value::Register(a), Value::Register(b))
                    if *a > REGISTERY_SIZE || *b > REGISTERY_SIZE =>
                {
                    return Err(anyhow!("invalid register in instruction {:?}", instr));
                }
                _ => {}
            }
        }

        Ok(Program {
            instructions,
            instruction_ptr: 0,
        })
    }

    pub fn run_one<F>(&mut self, registers: &mut Registers, mut transmit: F) -> bool
    where
        F: FnMut(i64) -> (),
    {
        if self.instruction_ptr < 0 || self.instruction_ptr >= self.instructions.len() as i64 {
            return false;
        }

        match &self.instructions[self.instruction_ptr as usize] {
            Instruction::Toggle(reg) => {
                let new_ptr = registers[*reg] + self.instruction_ptr;
                if new_ptr >= 0 && new_ptr < self.instructions.len() as i64 {
                    self.instructions[new_ptr as usize] =
                        self.instructions[new_ptr as usize].toggle();
                }
            }
            Instruction::Increment(reg) => registers[*reg] += 1,
            Instruction::Decrement(reg) => registers[*reg] -= 1,
            Instruction::Out(value) => transmit(value.resolve(registers)),
            Instruction::Copy(value, Value::Register(reg)) => {
                registers[*reg] = value.resolve(registers)
            }
            Instruction::Copy(_, Value::Constant(_)) => {}
            Instruction::JumpIfNotZero(condition, target) => {
                if condition.resolve(registers) != 0 {
                    self.instruction_ptr += target.resolve(registers);
                    return true;
                }
            }
        }
        self.instruction_ptr += 1;

        true
    }

    pub fn run_to_end<F>(&mut self, registers: &mut Registers, mut transmit: F)
    where
        F: FnMut(i64) -> (),
    {
        while self.run_one(registers, |v| transmit(v)) {}
    }
}

impl Instruction {
    pub fn toggle(self) -> Instruction {
        match self {
            Instruction::Toggle(x) | Instruction::Decrement(x) => Instruction::Increment(x),
            Instruction::Increment(x) => Instruction::Decrement(x),
            Instruction::Copy(x, y) => Instruction::JumpIfNotZero(x, y),
            Instruction::JumpIfNotZero(x, y) => Instruction::Copy(x, y),
            x @ Instruction::Out(_) => x,
        }
    }
}

impl Value {
    pub fn resolve(&self, registers: &Registers) -> i64 {
        match *self {
            Value::Constant(value) => value,
            Value::Register(index) => registers[index],
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use fmt::Write;
        for (idx, instr) in self.instructions.iter().enumerate() {
            f.write_char(if self.instruction_ptr == idx as i64 {
                '>'
            } else {
                ' '
            })?;
            match instr {
                Instruction::Toggle(v) => write!(f, "tgl {}\n", v),
                Instruction::Increment(v) => write!(f, "inc {}\n", v),
                Instruction::Decrement(v) => write!(f, "dec {}\n", v),
                Instruction::Out(v) => write!(f, "out {}\n", v),
                Instruction::Copy(a, b) => write!(f, "cpy {} {}\n", a, b),
                Instruction::JumpIfNotZero(a, b) => write!(f, "jnz {} {}\n", a, b),
            }?;
        }
        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use fmt::Write;
        match self {
            Value::Constant(c) => write!(f, "{}", c),
            Value::Register(r) => f.write_char((b'a' + *r as u8) as char),
        }
    }
}

pub fn parse_register(s: &str) -> nom::IResult<&str, usize> {
    use crate::parsers::*;
    map_res(anychar, |c| {
        if c >= 'a' && c <= 'z' {
            Ok((c as u8 - b'a') as usize)
        } else {
            Err(())
        }
    })(s)
}
pub fn parse_value(s: &str) -> nom::IResult<&str, Value> {
    use crate::parsers::*;
    alt((
        map(parse_register, Value::Register),
        map(i64_str, Value::Constant),
    ))(s)
}

pub fn parse_assembunny(s: &str) -> nom::IResult<&str, Vec<Instruction>> {
    use crate::parsers::*;
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
        map(preceded(tag("out "), parse_value), Instruction::Out),
        instr_2op("cpy", Instruction::Copy),
        instr_2op("jnz", Instruction::JumpIfNotZero),
    ));

    separated_list1(line_ending, instruction)(s)
}
