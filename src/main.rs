mod ast;
mod compiler;
mod instructions;
mod parser;
mod queue;
mod tokenizer;
mod vm;

use compiler::compile;
use instructions::Instruction;
use parser::parse_regex_tokens;
use tokenizer::tokenize;
use vm::thompson;

fn regex_to_program(regex: &str) -> Vec<Instruction> {
    let mut tokens = tokenize(regex);
    let instructions = parse_regex_tokens(&mut tokens);
    compile(&instructions)
}
fn main() {
    let regex = "(a|b)*cba?(c|b)bb";
    let input = "abcbabbb";
    let program = regex_to_program(regex);
    for (i, inst) in program.iter().enumerate() {
        println!("{i}\t {}\t", inst);
    }
    println!("{} => {} = {}", regex, input, thompson(program, input));
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_full_match {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, text, expected) = $value;
                assert_eq!(expected, full_match(input, text));
            }
        )*
        }
    }
    macro_rules! test_partial_match {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, text, expected) = $value;
                assert_eq!(expected, partial_match(input, text));
            }
        )*
        }
    }

    test_full_match! {
        full_match1: ("a?", "a", true),
        full_match2: ("a?", "b", false),
        full_match3: ("a?b?", "ab", true),
        full_match4: ("a?b?", "a", true),
        full_match5: ("a+b", "ab", true),
        full_match6: ("a+b", "a", false),
        full_match7: ("a+b?", "a", true),
        full_match8: ("(a+b+)", "ab", true),
        full_match9: ("a|b", "a", true),
        full_match10: ("a|b", "b", true),
        full_match11: ("a|b", "c", false),
        full_match12: ("aa|b", "aa", true),
        full_match13: ("a?a|b", "a", true),
        full_match14: ("a*b?", "a", true),
        full_match15: ("a*b?", "b", true),
        full_match16: ("(a|b)?", "a", true),
        full_match17: ("ab.a.a.ab.a", "abracadabra", true),
        full_match18: ("a.b.a", "a b a", true),
        full_match19: ("[abc,", "a", true),
        full_match20: ("[abc,", "A", false),
        full_match21: ("[^abc,", "a", false),
        full_match22: ("^a", "a", true),
        full_match23: ("^a", "b", false),
        full_match24: ("^a.", "ab", true),
        full_match25: ("^(a|b)", "a", true),
        full_match26: ("^(a|b)", "c", false),
        full_match27: ("^abc$", "abc", true),
    }
    test_partial_match! {
        partial_match1: ("a?", "aaaab", true),
        partial_match2: ("^a.+b", "aab", true),
        partial_match3: ("^a.+b", "acb", true),
        partial_match4: ("^a.+b", "acc", false),
        partial_match5: ("^lug", "slug", false),
        partial_match6: ("dog$", "dog", true),
        partial_match7: ("dog$", "dogs", false),
    }

    fn full_match(regex: &str, text: &str) -> bool {
        let program = regex_to_program(regex);
        println!("fully matching {} with {}", regex, text);
        for (i, inst) in program.iter().enumerate() {
            println!("{i}\t {}\t", inst);
        }
        println!("====");
        thompson(program, text)
    }
    fn partial_match(regex: &str, text: &str) -> bool {
        let program = regex_to_program(regex);
        println!("partial matching {} with {}", regex, text);
        for (i, inst) in program.iter().enumerate() {
            println!("{i}\t {}\t", inst);
        }
        println!("====");
        let mut i = 1;
        loop {
            if i > text.len() {
                return false;
            }
            let current_text = &text[..i];
            if thompson(program.clone(), current_text) {
                return true;
            }
            i += 1;
        }
    }
}
