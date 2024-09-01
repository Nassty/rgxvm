use std::collections::VecDeque;

use crate::{
    ast::Ast,
    tokenizer::{Token, TokenType},
};

fn parse_group<'a, T>(tokens: &mut T) -> Vec<Ast>
where
    T: Iterator<Item = &'a Token> + std::fmt::Debug + Clone,
{
    let mut instructions = VecDeque::new();
    loop {
        match tokens.next() {
            Some(Token {
                ty: TokenType::LeftParen,
                ..
            }) => {
                instructions.push_back(Ast::Group(parse_group(
                    &mut tokens
                        .take_while(|t| t.ty != TokenType::RightParen)
                        .collect::<Vec<_>>()
                        .iter()
                        .cloned(),
                )));
            }
            Some(Token {
                ty: TokenType::Pipe,
                ..
            }) => {
                let instr = Ast::Pipe(
                    Box::new(Ast::Group(instructions.into())),
                    Box::new(Ast::Group(parse_group(tokens))),
                );
                instructions = vec![instr].into();
            }
            Some(Token {
                ty: TokenType::Start,
                ..
            }) => instructions.push_back(Ast::Start(parse_group(
                &mut tokens.collect::<Vec<_>>().iter().cloned(),
            ))),
            Some(Token {
                ty: TokenType::End, ..
            }) => {
                let mut g = VecDeque::new();
                while instructions.iter().last().is_some() {
                    match instructions.iter().last().unwrap() {
                        Ast::Set(_) | Ast::Group(_) | Ast::Exact(_) => {
                            g.push_front(instructions.pop_back().unwrap())
                        }
                        _ => break,
                    }
                }
                instructions.push_back(Ast::End(g.into()));
            }
            Some(Token {
                ty: TokenType::LeftBrace,
                ..
            }) => {
                let mut exclude = false;
                let mut s = tokens.clone().peekable();
                if let Some(Token {
                    ty: TokenType::Start,
                    ..
                }) = s.peek()
                {
                    exclude = true;
                    tokens.next();
                }
                let els = parse_group(
                    &mut tokens
                        .take_while(|t| t.ty != TokenType::RightBrace)
                        .cloned()
                        .map(|mut t| {
                            if exclude && t.ty == TokenType::Char {
                                t.ty = TokenType::NChar;
                            }
                            t
                        })
                        .collect::<Vec<_>>()
                        .iter(),
                );
                if exclude {
                    instructions.push_back(Ast::Exclude(els));
                } else {
                    instructions.push_back(Ast::Set(els));
                }
            }
            Some(Token {
                ty: TokenType::Asterisk,
                ..
            }) => {
                let instr = Ast::Star(Box::new(instructions.pop_back().unwrap()));
                instructions.push_back(instr);
            }
            Some(Token {
                ty: TokenType::Plus,
                ..
            }) => {
                let instr = Ast::Plus(Box::new(instructions.pop_back().unwrap()));
                instructions.push_back(instr)
            }
            Some(Token {
                ty: TokenType::Question,
                ..
            }) => {
                let instr = Ast::Question(Box::new(instructions.pop_back().unwrap()));
                instructions.push_back(instr)
            }
            Some(Token {
                ty: TokenType::Char,
                value,
            }) => {
                instructions.push_back(Ast::Exact(value.to_string()));
            }
            Some(Token {
                ty: TokenType::Dot, ..
            }) => {
                instructions.push_back(Ast::Dot);
            }
            Some(Token {
                ty: TokenType::NChar,
                value,
            }) => {
                instructions.push_back(Ast::NExact(value.to_string()));
            }
            None => break,
            k => todo!("{:?}", k),
        }
    }
    instructions.into()
}

pub fn parse_regex_tokens(tokens: &mut [Token]) -> Vec<Ast> {
    parse_group(&mut tokens.iter())
}
