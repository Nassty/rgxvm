use crate::{
    instructions::{Instruction, Opcode},
    queue::Queue,
};

pub fn thompson(instructions: Vec<Instruction>, text: &str) -> bool {
    let mut cqueue = Queue::default();
    let mut nqueue = Queue::default();
    cqueue.push(0);

    for i in 0..=text.len() {
        while !cqueue.is_empty() {
            let pc = cqueue.pop().expect("empty queue");
            let inst = instructions.get(pc as usize).expect("invalid instruction");
            match inst.opcode {
                Opcode::Char => {
                    if text.chars().nth(i) == char::from_u32(inst.x) || inst.x == 0 {
                        nqueue.push(pc + 1);
                    }
                }
                Opcode::NChar => {
                    if text.chars().nth(i) != char::from_u32(inst.x) {
                        nqueue.push(pc + 1);
                    }
                }
                Opcode::Match => {
                    if i == text.len() {
                        return true;
                    }
                }
                Opcode::Split => {
                    cqueue.push(inst.x);
                    cqueue.push(inst.y);
                }
                Opcode::Jump => {
                    cqueue.push(inst.x);
                }
            }
        }
        std::mem::swap(&mut cqueue, &mut nqueue);
    }

    false
}
