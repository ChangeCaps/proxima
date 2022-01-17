use std::{env, fs};

use proxy::*;

const PRINT: u32 = 0;
const READ: u32 = 1;
const ASM: u32 = 2;

const ASM_OFFSET: u32 = 128;

#[derive(Default)]
struct State {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).unwrap();

    let source = fs::read_to_string(&path)?;

    let lines = proxy::parse_file(&source)?;
    let program = assemble_lines(lines)?;

    let mut cpu = Cpu::<State>::default();
    cpu.register_sys_call(PRINT, |cpu, _| {
        let ptr = cpu.registers.read(Register::EAX).to_u32();
        let len = cpu.registers.read(Register::EBX).to_u32();

        let string = cpu.memory.read_string(ptr, len).unwrap();

        println!("{}", string);
    });
    cpu.register_sys_call(READ, |cpu, _| {
        let path_ptr = cpu.registers.read(Register::EAX).to_u32();
        let path_len = cpu.registers.read(Register::EBX).to_u32();

        let path = cpu.memory.read_string(path_ptr, path_len).unwrap();

        let contents = fs::read(path.as_ref()).unwrap();
        let len = contents.len() as u32;

        assert!(len < cpu.abi().system_memory);

        cpu.memory.write_bytes(0, &contents);

        cpu.registers.write(Register::EAX, Word::from_u32(0));
        cpu.registers.write(Register::EBX, Word::from_u32(len));
    });
    cpu.register_sys_call(ASM, |cpu, _| {
        let source_ptr = cpu.registers.read(Register::EAX).to_u32();
        let source_len = cpu.registers.read(Register::EBX).to_u32();

        let source = cpu.memory.read_string(source_ptr, source_len).unwrap();

        let lines = proxy::parse_file(&source).unwrap();
        let program = assemble_lines(lines).unwrap();
        let len = program.len();

        assert!(ASM_OFFSET + len < cpu.abi().system_memory);

        cpu.memory.write_bytes(ASM_OFFSET, program.bytes());

        cpu.registers
            .write(Register::EAX, Word::from_u32(ASM_OFFSET));
        cpu.registers.write(Register::EBX, Word::from_u32(len));
    });
    cpu.load_program(&program);

    let mut state = State::default();

    cpu.run(&mut state);

    Ok(())
}
