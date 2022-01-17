use std::mem;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register(pub u8);

impl Register {
    pub const EAX: Self = Self::new(0);
    pub const EBX: Self = Self::new(1);
    pub const ECX: Self = Self::new(2);
    pub const EDX: Self = Self::new(3);
    pub const EIP: Self = Self::new(4);
    pub const ESP: Self = Self::new(5);
    pub const ERP: Self = Self::new(6);
    pub const EBP: Self = Self::new(7);
    pub const EXP: Self = Self::new(8);

    pub const MIN_REGISTERS: usize = 12;

    pub const fn new(index: u8) -> Self {
        Self(index)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Opcode(pub u8);

impl Opcode {
    pub const CONST: Self = Self(0);
    pub const MOV: Self = Self(1);
    pub const PUSH: Self = Self(2);
    pub const POP: Self = Self(3);
    pub const LOAD: Self = Self(4);
    pub const STORE: Self = Self(5);

    pub const JMP: Self = Self(16);
    pub const JMP_NZ: Self = Self(17);
    pub const CALL: Self = Self(18);
    pub const RET: Self = Self(19);
    pub const EXIT: Self = Self(20);

    pub const ADDI: Self = Self(32);
    pub const SUBI: Self = Self(33);
    pub const MULI: Self = Self(34);
    pub const DIVI: Self = Self(35);
    pub const MODI: Self = Self(36);
    pub const GTI: Self = Self(37);
    pub const LTI: Self = Self(38);

    pub const SHIFT: Self = Self(48);
    pub const AND: Self = Self(49);
    pub const OR: Self = Self(50);
    pub const XOR: Self = Self(51);
    pub const EQ: Self = Self(52);

    pub const ADDF: Self = Self(64);
    pub const SUBF: Self = Self(65);
    pub const MULF: Self = Self(66);
    pub const DIVF: Self = Self(67);
    pub const MODF: Self = Self(68);
    pub const FLOORF: Self = Self(72);
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word(pub [u8; 4]);

impl Word {
    pub const WIDTH: u8 = std::mem::size_of::<Self>() as u8;
    pub const SIZE: u32 = std::mem::size_of::<Self>() as u32;

    pub fn from_u32(value: u32) -> Self {
        Self(u32::to_be_bytes(value))
    }

    pub fn from_i32(value: i32) -> Self {
        Self(i32::to_be_bytes(value))
    }

    pub fn from_f32(value: f32) -> Self {
        unsafe { mem::transmute(value) }
    }

    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    pub fn to_u32(self) -> u32 {
        u32::from_be_bytes(self.0)
    }

    pub fn to_i32(self) -> i32 {
        i32::from_be_bytes(self.0)
    }

    pub fn to_f32(self) -> f32 {
        unsafe { mem::transmute(self) }
    }

    pub fn to_bytes(self) -> [u8; 4] {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Arg(pub u8);

impl From<Register> for Arg {
    fn from(reg: Register) -> Self {
        Self(reg.0)
    }
}

impl From<u8> for Arg {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl From<Arg> for Register {
    fn from(arg: Arg) -> Self {
        Register(arg.0)
    }
}

impl From<Arg> for u8 {
    fn from(arg: Arg) -> Self {
        arg.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Args {
    inner: [Arg; 3],
}

impl Args {
    pub fn from_bytes(args: [u8; 3]) -> Self {
        Self {
            inner: unsafe { mem::transmute(args) },
        }
    }

    pub fn arg<T: From<Arg>>(&self, index: usize) -> T {
        self.inner[index].into()
    }
}

impl From<()> for Args {
    fn from((): ()) -> Self {
        Self { inner: [Arg(0); 3] }
    }
}

impl<T: Into<Arg>> From<(T,)> for Args {
    fn from(args: (T,)) -> Self {
        Self {
            inner: [args.0.into(), Arg(0), Arg(0)],
        }
    }
}

impl<T: Into<Arg>, U: Into<Arg>> From<(T, U)> for Args {
    fn from(args: (T, U)) -> Self {
        Self {
            inner: [args.0.into(), args.1.into(), Arg(0)],
        }
    }
}

impl<T: Into<Arg>, U: Into<Arg>, V: Into<Arg>> From<(T, U, V)> for Args {
    fn from(args: (T, U, V)) -> Self {
        Self {
            inner: [args.0.into(), args.1.into(), args.2.into()],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub args: Args,
}

impl Instruction {
    pub fn from_word(word: Word) -> Self {
        let bytes = word.to_bytes();

        Self {
            opcode: Opcode(bytes[0]),
            args: Args::from_bytes([bytes[1], bytes[2], bytes[3]]),
        }
    }

    pub fn to_word(self) -> Word {
        Word::from_bytes([
            self.opcode.0,
            self.args.inner[0].0,
            self.args.inner[1].0,
            self.args.inner[2].0,
        ])
    }

    pub fn arg<T: From<Arg>>(&self, index: usize) -> T {
        self.args.arg(index)
    }
}
