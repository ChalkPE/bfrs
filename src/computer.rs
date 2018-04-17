use std::io::*;
use tokenizer::Token;

fn read() -> u8 {
    stdin().bytes().next().and_then(|r| r.ok()).unwrap()
}

fn write(data: u8) {
    stdout().write(&[data]).unwrap();
}

pub fn compute(program: &Vec<Token>, tape: &mut Vec<u8>) -> u8 {
    let mut code_pointer: usize = 0;
    let mut data_pointer: usize = 0;

    while code_pointer < program.len() {
        match program[code_pointer] {
            Token::Add => tape[data_pointer] += 1,
            Token::Sub => tape[data_pointer] -= 1,

            Token::Write => write(tape[data_pointer]),
            Token::Read => tape[data_pointer] = read(),

            Token::Prev => {
                if data_pointer > 0 { data_pointer -= 1 }
            },
            Token::Next => {
                data_pointer += 1;
                if data_pointer == tape.len() { tape.push(0); }
            },

            Token::Open => if tape[data_pointer] == 0 {
                let mut level = 1;
                while level > 0 {
                    code_pointer += 1;
                    match program[code_pointer] {
                        Token::Open => level += 1,
                        Token::Close => level -= 1,
                        _ => ()
                    }
                }
            },
            Token::Close => if tape[data_pointer] != 0 {
                let mut level = 1;
                while level > 0 {
                    code_pointer -= 1;
                    match program[code_pointer] {
                        Token::Open => level -= 1,
                        Token::Close => level += 1,
                        _ => ()
                    }
                }
            },
        }

        code_pointer += 1;
    }

    tape[data_pointer]
}
