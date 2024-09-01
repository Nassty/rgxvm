use crate::{ast::Ast, instructions::Instruction};

pub fn compile(tokens: &[Ast]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    compile_group(tokens, &mut instructions);
    instructions.push(Instruction::match_()); // End of pattern
    instructions
}

fn compile_group(tokens: &[Ast], output: &mut Vec<Instruction>) {
    for token in tokens {
        match token {
            Ast::Start(k) => {
                let split_index = output.len() as u32;
                output.push(Instruction::split(split_index + 1, 0));
                for instr in k {
                    compile_group(&[instr.clone()], output);
                }
                output[split_index as usize].y = output.len() as u32;
            }
            Ast::End(k) => {
                let split_index = output.len() as u32;
                output.push(Instruction::split(split_index + 1, 0));
                for instr in k {
                    compile_group(&[instr.clone()], output)
                }
                output[split_index as usize].y = output.len() as u32;
            }
            Ast::Set(chars) => {
                for ch in chars {
                    let current = output.len() as u32;
                    output.push(Instruction::split(current + 1, current + 2));
                    compile_group(&[ch.clone()], output)
                }
            }
            Ast::NExact(literal) => {
                for ch in literal.chars() {
                    output.push(Instruction::nchar(ch as u32));
                }
            }
            Ast::Exclude(excl) => {
                let mut currents = vec![];
                for ex in excl {
                    let current = output.len() as u32;
                    currents.push(current);
                    output.push(Instruction::split(0, current + 2));
                    compile_group(&[ex.clone()], output)
                }
                for current in currents {
                    output[current as usize].x = output.len() as u32;
                }
            }
            Ast::Exact(literal) => {
                for ch in literal.chars() {
                    output.push(Instruction::char(ch as u32));
                }
            }
            Ast::Star(inner) => {
                let start = output.len() as u32;
                output.push(Instruction::split(start + 1, 0));
                compile_group(&[*inner.clone()], output);
                output.push(Instruction::jump(start));
                output[start as usize].y = (output.len() + 1) as u32;
                output.push(Instruction::match_());
            }
            Ast::Plus(inner) => {
                let start = output.len() as u32;
                compile_group(&[*inner.clone()], output);
                output.push(Instruction::split(start, output.len() as u32 + 1));
            }
            Ast::Question(inner) => {
                let split_index = output.len() as u32;
                output.push(Instruction::split(split_index + 1, 0));
                compile_group(&[*inner.clone()], output);
                output[split_index as usize].y = output.len() as u32;
            }
            Ast::Dot => {
                output.push(Instruction::char(0)); // '.' can match any char
            }
            Ast::Pipe(left, right) => {
                let split_index = output.len();
                output.push(Instruction::split(split_index as u32 + 1, 0));

                compile_group(&[*left.clone()], output);
                let jump_index = output.len() as u32;
                let right_start = output.len();
                output.push(Instruction::jump(0));
                compile_group(&[*right.clone()], output);

                output[split_index].y = jump_index + 1;
                output[right_start].x = output.len() as u32;
            }
            Ast::Group(group) => {
                compile_group(group, output);
            }
        }
    }
}
