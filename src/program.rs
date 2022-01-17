use crate::{Instruction, Word};

pub struct Program {
    data: Vec<u8>,
}

impl Program {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push_word(&mut self, word: Word) {
        let bytes = word.to_bytes();
        self.data.extend(bytes);
    }

    pub fn push_instruction(&mut self, ins: Instruction) {
        self.push_word(ins.to_word())
    }

    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
}
