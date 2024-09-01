use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Opcode {
    Char,
    NChar,
    Split,
    Jump,
    Match,
}

#[derive(Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub x: u32,
    pub y: u32,
}

impl Instruction {
    pub fn new(opcode: Opcode, x: u32, y: u32) -> Self {
        Self { opcode, x, y }
    }
    pub fn as_rust_code(&self) -> String {
        match self.opcode {
            Opcode::Char => format!(
                "Instruction::char('{}' as u32)",
                char::from_u32(self.x).unwrap()
            ),
            Opcode::NChar => format!(
                "Instruction::nchar('{}' as u32)",
                char::from_u32(self.x).unwrap()
            ),
            Opcode::Match => "Instruction::match_()".to_string(),
            Opcode::Jump => format!("Instruction::jump({})", self.x),
            Opcode::Split => format!("Instruction::split({}, {})", self.x, self.y),
        }
    }

    pub fn char(x: u32) -> Self {
        Self::new(Opcode::Char, x, 0)
    }
    pub fn nchar(x: u32) -> Self {
        Self::new(Opcode::NChar, x, 0)
    }
    pub fn split(x: u32, y: u32) -> Self {
        Self::new(Opcode::Split, x, y)
    }
    pub fn jump(x: u32) -> Self {
        Self::new(Opcode::Jump, x, 0)
    }
    pub fn match_() -> Self {
        Self::new(Opcode::Match, 0, 0)
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.opcode {
                Opcode::Char => format!("CHAR\t{}", char::from_u32(self.x).unwrap()),
                Opcode::NChar => format!("NCHAR\t{}", char::from_u32(self.x).unwrap()),
                Opcode::Split => format!("SPLIT\t{},{}", self.x, self.y),
                Opcode::Jump => format!("JUMP\t{}", self.x),
                Opcode::Match => "MATCH".to_string(),
            }
        )
    }
}
