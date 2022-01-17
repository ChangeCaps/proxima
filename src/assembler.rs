use std::{borrow::Cow, collections::HashMap};

use crate::{Arg, Args, Instruction, Label, Opcode, Program, Register, Word};

#[derive(Clone, Debug)]
pub struct AssemblerError {
    message: Cow<'static, str>,
}

impl AssemblerError {
    pub fn new(msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for AssemblerError {}

#[derive(Clone, Debug)]
pub enum Constant {
    Literal(Word),
    Label(Label),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Line {
    Comment(String),
    Label(Label),
    Constant { constant: Constant, dst: Register },
    Instruction(Instruction),
}

fn parse_label(label: &str) -> Result<Label, AssemblerError> {
    let label = label.trim();

    if label.is_empty() {
        return Err(AssemblerError::new("invalid label"));
    }

    for ch in label.chars() {
        if ch.is_whitespace() || ch.is_control() || ch == '"' {
            return Err(AssemblerError::new("invalid label"));
        }
    }

    Ok(Label::new(label))
}

fn arg<'a>(args: &[&'a str], index: usize) -> Result<&'a str, AssemblerError> {
    if index >= args.len() {
        return Err(AssemblerError::new(format!("expected arg '{}'", index)));
    }

    Ok(args[index])
}

fn parse_i32(src: &str) -> Result<i32, AssemblerError> {
    if let Some(value) = src.strip_suffix('i') {
        if let Ok(value) = value.parse::<i32>() {
            Ok(value)
        } else {
            Err(AssemblerError::new("expected i32"))
        }
    } else {
        Err(AssemblerError::new("expected 'i' suffix"))
    }
}

fn parse_u32(src: &str) -> Result<u32, AssemblerError> {
    if let Some(value) = src.strip_suffix('u') {
        if let Ok(value) = value.parse::<u32>() {
            Ok(value)
        } else {
            Err(AssemblerError::new("expected u32"))
        }
    } else {
        Err(AssemblerError::new("expected 'u' suffix"))
    }
}

fn parse_f32(src: &str) -> Result<f32, AssemblerError> {
    if let Some(value) = src.strip_suffix('f') {
        if let Ok(value) = value.parse::<f32>() {
            Ok(value)
        } else {
            Err(AssemblerError::new("expected f32"))
        }
    } else {
        Err(AssemblerError::new("expected 'u' suffix"))
    }
}

fn parse_word(src: &str) -> Result<Word, AssemblerError> {
    if let Ok(value) = parse_u32(src) {
        return Ok(Word::from_u32(value));
    }

    if let Ok(value) = parse_i32(src) {
        return Ok(Word::from_i32(value));
    }

    if let Ok(value) = parse_f32(src) {
        return Ok(Word::from_f32(value));
    }

    Err(AssemblerError::new("expected word"))
}

fn parse_register(src: &str) -> Result<Register, AssemblerError> {
    match src {
        "eax" => return Ok(Register::EAX),
        "ebx" => return Ok(Register::EBX),
        "ecx" => return Ok(Register::ECX),
        "edx" => return Ok(Register::EDX),
        "eip" => return Ok(Register::EIP),
        "esp" => return Ok(Register::ESP),
        "erp" => return Ok(Register::ERP),
        "ebp" => return Ok(Register::EBP),
        "exp" => return Ok(Register::EXP),
        _ => {}
    }

    if let Some(index) = src.strip_prefix('%') {
        if let Ok(index) = index.parse::<u8>() {
            Ok(Register::new(index))
        } else {
            Err(AssemblerError::new("expected u8"))
        }
    } else {
        Err(AssemblerError::new("expected '%' prefix"))
    }
}

fn parse_width(src: &str) -> Result<u8, AssemblerError> {
    if let Ok(width) = src.parse::<u8>() {
        Ok(width)
    } else {
        Err(AssemblerError::new("expected u8"))
    }
}

fn parse_string(src: &str) -> Result<String, AssemblerError> {
    if let Some(string) = src
        .strip_prefix('"')
        .and_then(|string| string.strip_suffix('"'))
    {
        Ok(String::from(string))
    } else {
        Err(AssemblerError::new("invalid string"))
    }
}

fn parse_constant(src: &str) -> Result<Constant, AssemblerError> {
    if let Ok(word) = parse_word(src) {
        Ok(Constant::Literal(word))
    } else if let Ok(string) = parse_string(src) {
        Ok(Constant::String(string))
    } else {
        Ok(Constant::Label(parse_label(src)?))
    }
}

#[allow(unused_assignments)]
fn parse_instruction(instruction: &str, args: &[&str]) -> Result<Line, AssemblerError> {
    macro_rules! ins {
        ($opcode:ident, [$($arg:tt),*]) => {
            Instruction {
                opcode: Opcode::$opcode,
                args: {
                    #[allow(unused)]
                    let mut arg = 0usize;

                    Args::from(($({
                        let res = match stringify!($arg) {
                            "reg" => Arg::from(parse_register(self::arg(args, arg)?)?),
                            "width" => Arg::from(parse_width(self::arg(args, arg)?)?),
                            _ => panic!(),
                        };

                        arg += 1;

                        res
                    },)*))
                },
            }
        };
    }

    Ok(Line::Instruction(match instruction {
        "const" => {
            return Ok(Line::Constant {
                constant: parse_constant(arg(args, 0)?)?,
                dst: parse_register(arg(args, 1)?)?,
            });
        }

        "mov" => ins!(MOV, [reg, reg]),
        "push" => ins!(PUSH, [reg]),
        "pop" => ins!(POP, [reg]),
        "load" => ins!(LOAD, [reg, reg, width]),
        "store" => ins!(STORE, [reg, reg, width]),

        "jmp" => ins!(JMP, [reg]),
        "jmpnz" => ins!(JMP_NZ, [reg, reg]),
        "call" => ins!(CALL, [reg]),
        "ret" => ins!(RET, []),
        "exit" => ins!(EXIT, [reg]),

        "addi" => ins!(ADDI, [reg, reg, reg]),
        "subi" => ins!(SUBI, [reg, reg, reg]),
        "muli" => ins!(MULI, [reg, reg, reg]),
        "divi" => ins!(DIVI, [reg, reg, reg]),
        "modi" => ins!(MODI, [reg, reg, reg]),
        "gti" => ins!(GTI, [reg, reg, reg]),
        "lti" => ins!(LTI, [reg, reg, reg]),

        "shift" => ins!(SHIFT, [reg, reg, reg]),
        "and" => ins!(AND, [reg, reg, reg]),
        "or" => ins!(OR, [reg, reg, reg]),
        "xor" => ins!(XOR, [reg, reg, reg]),
        "eq" => ins!(EQ, [reg, reg, reg]),

        "addf" => ins!(ADDF, [reg, reg, reg]),
        "subf" => ins!(SUBF, [reg, reg, reg]),
        "mulf" => ins!(MULF, [reg, reg, reg]),
        "divf" => ins!(DIVF, [reg, reg, reg]),
        "modf" => ins!(MODF, [reg, reg, reg]),
        "floorf" => ins!(FLOORF, [reg, reg]),
        _ => {
            return Err(AssemblerError::new(format!(
                "invalid instruction {}",
                instruction
            )))
        }
    }))
}

fn parse_line(line: &str) -> Result<Line, AssemblerError> {
    if let Some(comment) = line.strip_prefix("//") {
        return Ok(Line::Comment(comment.to_owned()));
    }

    if let Some(label) = line.strip_suffix(':') {
        Ok(Line::Label(parse_label(label)?))
    } else {
        let mut parts = line.trim().split_whitespace();

        if let Some(instruction) = parts.next() {
            let args = parts.collect::<Vec<_>>();

            Ok(parse_instruction(instruction, &args)?)
        } else {
            Err(AssemblerError::new("expected instruction"))
        }
    }
}

pub fn parse_file(source: &str) -> Result<Vec<Line>, AssemblerError> {
    let mut lines = Vec::new();

    for line in source.trim().split('\n') {
        if !line.trim().is_empty() {
            lines.push(parse_line(line.trim())?);
        }
    }

    Ok(lines)
}

pub(crate) const fn align(ptr: u32, align: u32) -> u32 {
    (ptr - 1) / align * align + align
}

pub fn assemble_lines(lines: Vec<Line>) -> Result<Program, AssemblerError> {
    let mut labels = HashMap::new();

    let mut ins_offset = 0;

    for line in &lines {
        match line {
            Line::Label(label) => {
                if labels.insert(label.clone(), ins_offset).is_some() {
                    return Err(AssemblerError::new(format!(
                        "duplicate label '{}'",
                        label.0
                    )));
                }
            }
            Line::Constant { .. } => {
                ins_offset += 8;
            }
            Line::Instruction(_) => ins_offset += 4,
            _ => {}
        }
    }

    let mut const_offset = 0;
    let mut program = Program::new();

    for line in &lines {
        match line {
            Line::Constant { constant, dst } => {
                let ins = Instruction {
                    opcode: Opcode::CONST,
                    args: Args::from_bytes([dst.0, 0, 0]),
                };

                let data = match constant {
                    Constant::Label(label) => {
                        if let Some(offset) = labels.get(label) {
                            Word::from_u32(*offset)
                        } else {
                            return Err(AssemblerError::new(format!(
                                "undefined label '{}'",
                                label.0
                            )));
                        }
                    }
                    Constant::String(string) => {
                        let data = Word::from_u32(ins_offset + const_offset);

                        const_offset += align(string.len() as u32, 4) + 4;

                        data
                    }
                    &Constant::Literal(data) => data,
                };

                program.push_instruction(ins);
                program.push_word(data);
            }
            &Line::Instruction(ins) => {
                program.push_instruction(ins);
            }
            _ => {}
        }
    }

    for line in lines {
        match line {
            Line::Constant { constant, .. } => match constant {
                Constant::String(string) => {
                    program.push_word(Word::from_u32(string.len() as u32));

                    let mut bytes = string.bytes();

                    for _ in 0..align(string.len() as u32, 4) / 4 {
                        let data = Word::from_bytes([
                            bytes.next().unwrap_or(0),
                            bytes.next().unwrap_or(0),
                            bytes.next().unwrap_or(0),
                            bytes.next().unwrap_or(0),
                        ]);

                        program.push_word(data);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(program)
}
