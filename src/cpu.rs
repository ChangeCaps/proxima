use std::collections::HashMap;

use crate::{Instruction, Memory, Opcode, Program, Register, Word};

pub struct Registers {
    registers: Vec<Word>,
}

impl Registers {
    pub fn new(count: usize) -> Self {
        assert!(count > Register::MIN_REGISTERS);

        Self {
            registers: vec![Word::default(); count],
        }
    }

    pub fn read(&self, reg: Register) -> Word {
        self.registers[reg.0 as usize]
    }

    pub fn try_read(&self, reg: Register) -> Option<Word> {
        self.registers.get(reg.0 as usize).cloned()
    }

    pub fn write(&mut self, reg: Register, data: Word) {
        if let Some(reg) = self.registers.get_mut(reg.0 as usize) {
            *reg = data;
        }
    }

    pub fn eip(&self) -> Word {
        self.read(Register::EIP)
    }

    pub fn esp(&self) -> Word {
        self.read(Register::ESP)
    }

    pub fn erp(&self) -> Word {
        self.read(Register::ERP)
    }

    pub fn ebp(&self) -> Word {
        self.read(Register::EBP)
    }

    pub fn write_eip(&mut self, data: Word) {
        self.write(Register::EIP, data);
    }

    pub fn write_esp(&mut self, data: Word) {
        self.write(Register::ESP, data);
    }

    pub fn write_erp(&mut self, data: Word) {
        self.write(Register::ERP, data);
    }

    pub fn write_ebp(&mut self, data: Word) {
        self.write(Register::EBP, data);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Abi {
    pub register_count: u32,
    pub system_memory: u32,
    pub memory_size: u32,
}

impl Default for Abi {
    fn default() -> Self {
        Self {
            register_count: 16,
            system_memory: 2 << 12,
            memory_size: 2 << 16,
        }
    }
}

pub struct CpuState<'a> {
    abi: &'a Abi,
    pub registers: &'a mut Registers,
    pub memory: &'a mut Memory,
}

impl<'a> CpuState<'a> {
    pub fn abi(&self) -> &Abi {
        self.abi
    }
}

pub struct Cpu<T = ()> {
    abi: Abi,
    registers: Registers,
    memory: Memory,
    sys_calls: HashMap<u32, fn(&mut CpuState, &mut T)>,
}

impl<T> Default for Cpu<T> {
    fn default() -> Self {
        Self::new(Abi::default())
    }
}

impl<T> Cpu<T> {
    pub fn new(abi: Abi) -> Self {
        Self {
            abi,
            registers: Registers::new(abi.register_count as usize),
            memory: Memory::with_size(abi.memory_size as usize),
            sys_calls: HashMap::new(),
        }
    }

    pub fn abi(&self) -> &Abi {
        &self.abi
    }

    pub fn register_sys_call(&mut self, address: u32, call: fn(&mut CpuState, &mut T)) {
        self.sys_calls.insert(address, call);
    }

    pub fn push_stack(&mut self, data: Word) {
        let esp = self.registers.esp().to_u32();

        self.memory.write(data, esp, Word::WIDTH);

        self.registers
            .write_esp(Word::from_u32(esp + Word::WIDTH as u32));
    }

    pub fn pop_stack(&mut self) -> Word {
        let esp = self.registers.esp().to_u32() - Word::SIZE;
        self.registers.write_esp(Word::from_u32(esp));

        self.memory.read(esp, Word::WIDTH).unwrap()
    }

    pub fn load_program(&mut self, program: &Program) {
        self.memory
            .write_bytes(self.abi.system_memory, program.bytes());

        self.registers
            .write_eip(Word::from_u32(self.abi.system_memory));

        self.registers
            .write_esp(Word::from_u32(self.abi.system_memory + program.len()));

        self.registers
            .write_ebp(Word::from_u32(self.abi.system_memory));
    }

    pub fn eval_instruction(&mut self, state: &mut T) -> bool {
        let eip = self.registers.eip().to_u32();

        if let Some(sys_call) = self.sys_calls.get(&eip) {
            let mut cpu_state = CpuState {
                abi: &self.abi,
                registers: &mut self.registers,
                memory: &mut self.memory,
            };

            // run the sys_call
            sys_call(&mut cpu_state, state);

            // write erp to eip
            let erp = self.registers.erp();
            self.registers.write_eip(erp);

            return true;
        }

        let ins = Instruction::from_word(self.memory.read(eip, Word::WIDTH).unwrap());

        if ins.opcode != Opcode::CONST {
            self.registers.write_eip(Word::from_u32(eip + Word::SIZE));
        }

        match ins.opcode {
            Opcode::CONST => {
                let dst: Register = ins.arg(0);

                // read data
                let data = self.memory.read(eip + Word::SIZE, Word::WIDTH).unwrap();

                // write data
                self.registers.write(dst, data);

                // increment eip twice (to jump over the data)
                self.registers
                    .write_eip(Word::from_u32(eip + Word::SIZE * 2));
            }
            Opcode::MOV => {
                let src: Register = ins.arg(0);
                let dst: Register = ins.arg(1);

                // write %src to %dst
                let data = self.registers.read(src);
                self.registers.write(dst, data);
            }
            Opcode::PUSH => {
                let src: Register = ins.arg(0);

                // read %src
                let data = self.registers.read(src);

                self.push_stack(data);
            }
            Opcode::POP => {
                let dst: Register = ins.arg(0);

                let data = self.pop_stack();

                self.registers.write(dst, data);
            }
            Opcode::LOAD => {
                let src: Register = ins.arg(0);
                let dst: Register = ins.arg(1);
                let width: u8 = ins.arg(2);

                // load %src and read data @src
                let ptr = self.registers.read(src).to_u32();
                let data = self.memory.read(ptr, width).unwrap();

                // write data to %dst
                self.registers.write(dst, data);
            }
            Opcode::STORE => {
                let src: Register = ins.arg(0);
                let dst: Register = ins.arg(1);
                let width: u8 = ins.arg(2);

                // read %src and %dst
                let data = self.registers.read(src);
                let ptr = self.registers.read(dst).to_u32();

                // write %src to @dst
                self.memory.write(data, ptr, width);
            }
            Opcode::JMP => {
                let trg: Register = ins.arg(0);

                // read %trg
                let ptr = self.registers.read(trg);

                // write %trg to eip
                self.registers.write_eip(ptr);
            }
            Opcode::JMP_NZ => {
                let trg: Register = ins.arg(0);
                let src: Register = ins.arg(1);

                // read %src
                let data = self.registers.read(src).to_u32();

                if data != 0 {
                    // read %trg
                    let trg = self.registers.read(trg);

                    // write %trg to eip
                    self.registers.write_eip(trg);
                }
            }
            Opcode::CALL => {
                let trg: Register = ins.arg(0);

                // read %trg
                let trg = self.registers.read(trg);

                // write eip to erp
                let eip = self.registers.eip();
                self.registers.write_erp(eip);

                // write %trg to eip
                self.registers.write_eip(trg);
            }
            Opcode::RET => {
                // write erp to eip
                let erp = self.registers.erp();
                self.registers.write_eip(erp);
            }
            Opcode::EXIT => {
                let src: Register = ins.arg(0);

                let exit_code = self.registers.read(src).to_u32();

                println!("exited with ({})", exit_code);

                return false;
            }
            Opcode::ADDI => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs + %rhs to %dst
                self.registers.write(dst, Word::from_u32(lhs + rhs));
            }
            Opcode::SUBI => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs - %rhs to %dst
                self.registers.write(dst, Word::from_u32(lhs - rhs));
            }
            Opcode::MULI => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs * %rhs to %dst
                self.registers.write(dst, Word::from_u32(lhs * rhs));
            }
            Opcode::DIVI => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs / %rhs to %dst
                self.registers.write(dst, Word::from_u32(lhs / rhs));
            }
            Opcode::GTI => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs > %rhs to %dst
                self.registers
                    .write(dst, Word::from_u32((lhs > rhs) as u32));
            }
            Opcode::LTI => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs < %rhs to %dst
                self.registers
                    .write(dst, Word::from_u32((lhs < rhs) as u32));
            }

            Opcode::AND => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs & %rhs to %dst
                self.registers.write(dst, Word::from_u32(lhs & rhs));
            }
            Opcode::OR => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs | %rhs to %dst
                self.registers.write(dst, Word::from_u32(lhs | rhs));
            }
            Opcode::XOR => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs).to_u32();
                let rhs = self.registers.read(rhs).to_u32();

                // write %lhs ^ %rhs to %dst
                self.registers.write(dst, Word::from_u32(lhs ^ rhs));
            }
            Opcode::EQ => {
                let lhs: Register = ins.arg(0);
                let rhs: Register = ins.arg(1);
                let dst: Register = ins.arg(2);

                // read %lhs and %rhs
                let lhs = self.registers.read(lhs);
                let rhs = self.registers.read(rhs);

                // write %lhs == %rhs to %dst
                self.registers
                    .write(dst, Word::from_u32((lhs == rhs) as u32));
            }
            _ => {
                panic!("invalid opcode {}", ins.opcode.0);
            }
        }

        true
    }

    pub fn run(&mut self, state: &mut T) {
        loop {
            let running = self.eval_instruction(state);

            if !running {
                break;
            }
        }
    }
}
