use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    instructions: Vec<Instruction>,
    instruction_ptr: i64,
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

    pub fn run_one(&mut self, registers: &mut Registers) -> bool {
        if self.instruction_ptr < 0 || self.instruction_ptr >= self.instructions.len() as i64 {
            return false;
        }

        match &self.instructions[self.instruction_ptr as usize] {
            Instruction::Toggle(reg) => {
                let new_ptr = registers[*reg] + self.instruction_ptr;
                if new_ptr >= 0 && new_ptr < self.instructions.len() as i64 {
                    self.instructions[new_ptr as usize] =
                        self.instructions[new_ptr as usize].toggle();
                } else {
                    panic!("toggle was out of range");
                }
            }
            Instruction::Increment(reg) => registers[*reg] += 1,
            Instruction::Decrement(reg) => registers[*reg] -= 1,
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

    pub fn run_to_end(&mut self, registers: &mut Registers) {
        while self.run_one(registers) {}
    }
}

impl Instruction {
    pub fn toggle(self) -> Instruction {
        match self {
            Instruction::Toggle(x) | Instruction::Decrement(x) => Instruction::Increment(x),
            Instruction::Increment(x) => Instruction::Decrement(x),
            Instruction::Copy(x, y) => Instruction::JumpIfNotZero(x, y),
            Instruction::JumpIfNotZero(x, y) => Instruction::Copy(x, y),
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
