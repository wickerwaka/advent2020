use std::collections::HashSet;

use advent::*;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl AdventParse for Instruction {
    fn parse(line: &str) -> Result<Instruction, Error> {
        let v = line.split_whitespace().collect::<Vec<_>>();
        match v.as_slice() {
            ["nop", x] => Ok(Instruction::Nop(x.parse()?)),
            ["acc", x] => Ok(Instruction::Acc(x.parse()?)),
            ["jmp", x] => Ok(Instruction::Jmp(x.parse()?)),
            _ => Err(anyhow!("Invalid instruction")),
        }
    }
}

struct VirtualMachine {
    insts: Vec<Instruction>,
    pc: i64,
    acc: i64,
}

impl VirtualMachine {
    fn init(insts: Vec<Instruction>) -> VirtualMachine {
        VirtualMachine {
            insts,
            pc: 0,
            acc: 0,
        }
    }

    fn tick(&mut self) -> Result<(), Error> {
        //dbg!( self.insts[self.pc as usize]);
        match self.insts[self.pc as usize] {
            Instruction::Nop(_) => self.pc += 1,
            Instruction::Acc(x) => {
                self.acc += x;
                self.pc += 1;
            }
            Instruction::Jmp(x) => self.pc += x,
        }

        Ok(())
    }
}

fn replace_jmp_or_nop(insts: &[Instruction], nth: usize) -> Option<Vec<Instruction>> {
    let replacement = match insts[nth] {
        Instruction::Nop(x) => Some(Instruction::Jmp(x)),
        Instruction::Jmp(x) => Some(Instruction::Nop(x)),
        _ => None,
    };

    if let Some(replacement) = replacement {
        let mut v = Vec::from(insts);
        v[nth] = replacement;
        Some(v)
    } else {
        None
    }
}

fn main() -> Result<(), Error> {
    let insts: Vec<Instruction> = read_list("day08/input.txt")?;

    let mut vm = VirtualMachine::init(insts.clone());

    let mut pcs = HashSet::new();
    while pcs.insert(vm.pc) {
        vm.tick()?;
    }

    println!("Acc: {}", vm.acc);

    let inst_count = insts.len();
    for index in 0..inst_count {
        if let Some(insts) = replace_jmp_or_nop(&insts, index) {
            let mut vm = VirtualMachine::init(insts);
            let mut pcs = HashSet::new();
            while pcs.insert(vm.pc) && vm.pc != inst_count as i64 {
                vm.tick()?;
            }

            if vm.pc == inst_count as i64 {
                println!("Acc: {}", vm.acc);
            }
        }
    }

    Ok(())
}
