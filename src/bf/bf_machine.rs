use std::{
    error::Error,
    fmt::Display,
    io::{Read, Write},
};

use super::bf_token::BfToken;

#[derive(Debug)]
pub struct BfMachine<'a, W: Write, R: Read> {
    cursor: usize,
    memory: &'a mut [u8],
    output: &'a mut Box<W>,
    input: &'a mut Box<R>,
}

impl<'a, W: Write, R: Read> BfMachine<'a, W, R> {
    pub fn new(memory: &'a mut [u8], output: &'a mut Box<W>, input: &'a mut Box<R>) -> Self {
        Self {
            cursor: 0,
            memory,
            output,
            input,
        }
    }

    pub fn run(&mut self, commands: &[BfToken]) -> Result<(), Box<dyn Error>> {
        let mut program_counter = 0;
        let mut loop_stack = vec![];
        let mut skipping = false;
        let mut skip_loop_len = 0;

        while program_counter < commands.len() {
            match commands[program_counter] {
                token if token != BfToken::LoopStart && token != BfToken::LoopEnd && skipping => {}
                BfToken::NotCommand(_) => {}
                BfToken::Increment(val) => {
                    self.memory[self.cursor] = self.memory[self.cursor].wrapping_add(val);
                }
                BfToken::Decrement(val) => {
                    self.memory[self.cursor] = self.memory[self.cursor].wrapping_sub(val);
                }
                BfToken::CursorLeft(val) => {
                    if self.cursor == 0 {
                        self.cursor = self.memory.len();
                    }
                    self.cursor -= val % self.memory.len();
                }
                BfToken::CursorRight(val) => {
                    self.cursor += val % self.memory.len();
                }
                BfToken::LoopStart => {
                    if self.memory[self.cursor] == 0 && !skipping {
                        skipping = true;
                        skip_loop_len = loop_stack.len();
                    }

                    loop_stack.push(program_counter);
                }
                BfToken::LoopEnd => match loop_stack.pop() {
                    Some(pc) => {
                        if skipping {
                            skipping = loop_stack.len() != skip_loop_len;
                        } else {
                            program_counter = pc;
                            continue;
                        }
                    }
                    None => return Err(Box::new(BfRuntimeError::LoopNotClosed(program_counter))),
                },
                BfToken::PrintChar => {
                    self.output.write(&vec![self.memory[self.cursor]])?;
                }
                BfToken::InputChar => {
                    let mut input = [0; 1];
                    self.input.read_exact(&mut input)?;
                    self.memory[self.cursor] = input[0];
                }
            }

            program_counter += 1;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BfRuntimeError {
    LoopNotClosed(usize),
}

impl Display for BfRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::LoopNotClosed(pc) => {
                format!("The error occurred at the {pc}th character due to unclosed loop.")
            }
        };
        write!(f, "{message}")
    }
}

impl Error for BfRuntimeError {}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::bf::bf_parser::BfParser;

    use super::*;

    #[test]
    fn hello_world() {
        let mut memory = vec![0; 255];
        let mut output = Box::new(vec![0; 255]);
        let input_supply = [];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let commands = BfParser::parse(
            "++++++++++[>+++++++>++++++++++>+++>+<<<<-]
    >++.>+.+++++++..+++.>++.<<+++++++++++++++.
    >.+++.------.--------.>+.>.",
        );
        machine.run(&commands).unwrap();

        let mut result = Box::new(vec![0; 255]);
        result.write("Hello World!\n".as_bytes()).unwrap();

        assert_eq!(output, result);
    }

    #[test]
    fn clear_cursor_memory() {
        let mut memory = vec![0; 255];
        memory[0] = 10;
        let mut output = Box::new(vec![0; 255]);
        let input_supply = [];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let commands = BfParser::parse("[-]");

        assert_eq!(machine.memory[0], 10);

        machine.run(&commands).unwrap();

        assert_eq!(machine.memory[0], 0);
    }

    #[test]
    fn input_and_output() {
        let mut memory = vec![0; 255];
        let mut output = Box::new(vec![0; 255]);
        let input_supply = vec!['t' as u8];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let commands = BfParser::parse(",.");
        machine.run(&commands).unwrap();

        let mut result = Box::new(vec![0; 255]);
        result.write("t".as_bytes()).unwrap();

        assert_eq!(output, result);
    }

    #[test]
    fn run_batch_commands() {
        let mut memory = vec![0; 255];
        let mut output = Box::new(vec![0; 255]);
        let input_supply = vec![];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let assign_cell_0_to_10 = BfParser::parse("[-]++++++++++");
        let move_cell_0_to_cell_1 = BfParser::parse("[>+ <-]");
        let clear_cell_1 = BfParser::parse(">[-]");

        machine.run(&assign_cell_0_to_10).unwrap();
        assert_eq!(machine.memory[0], 10);

        machine.run(&move_cell_0_to_cell_1).unwrap();
        assert_eq!(machine.memory[0], 0);
        assert_eq!(machine.memory[1], 10);

        machine.run(&clear_cell_1).unwrap();
        assert_eq!(machine.memory[1], 0);
    }

    #[test]
    fn four_by_four() {
        let mut memory = vec![0; 255];
        let mut output = Box::new(vec![0; 255]);
        let input_supply = vec![];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let commands = BfParser::parse(
            "++++>++++>[-]>[-]>[-]<<<<[->[->+>+<<]>>[-<<+>>]>+<<<<]>>>>[-<<<<+>>>>]<<<<",
        );
        machine.run(&commands).unwrap();

        assert_eq!(machine.memory[0], 4);
        assert_eq!(machine.memory[1], 4);
        assert_eq!(machine.memory[2], 16);
    }

    #[test]
    fn pi() {
        let mut memory = vec![0; 255];
        let mut output = Box::new(vec![0; 255]);
        let input_supply = vec![];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let commands = BfParser::parse(
            ">+++++++++++++++[<+>>>>>>>>++++++++++<<<<<<<-]>+++++[<+++++++++>-]+>>>>>>+[<<+++
            [>>[-<]<[>]<-]>>[>+>]<[<]>]>[[->>>>+<<<<]>>>+++>-]<[<<<<]<<<<<<<<+[->>>>>>>>>>>>
            [<+[->>>>+<<<<]>>>>>]<<<<[>>>>>[<<<<+>>>>-]<<<<<-[<<++++++++++>>-]>>>[<<[<+<<+>>
            >-]<[>+<-]<++<<+>>>>>>-]<<[-]<<-<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>+<<<-[>>+<<
            -]<]<<<<+>>>>>>>>[-]>[<<<+>>>-]<<++++++++++<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>
            +>[<<+<+>>>-]<<<<+<+>>[-[-[-[-[-[-[-[-[-<->[-<+<->>]]]]]]]]]]<[+++++[<<<++++++++
            <++++++++>>>>-]<<<<+<->>>>[>+<<<+++++++++<->>>-]<<<<<[>>+<<-]+<[->-<]>[>>.<<<<[+
            .[-]]>>-]>[>>.<<-]>[-]>[-]>>>[>>[<<<<<<<<+>>>>>>>>-]<<-]]>>[-]<<<[-]<<<<<<<<]+++
            +++++++.",
        );
        machine.run(&commands).unwrap();

        let mut result = Box::new(vec![0; 255]);
        result.write("3.14070455282885\n".as_bytes()).unwrap();

        assert_eq!(output, result);
    }

    #[test]
    fn quine() {
        let mut memory = vec![0; 32767];
        let mut output = Box::new(vec![0; 255]);
        let input_supply = vec![];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let commands_literally =
            "-->+++>+>+>+>+++++>++>++>->+++>++>+>>>>>>>>>>>>>>>>->++++>>>>->+++>+++>+++>+++>+++>+++>+>+>>>->->>++++>+>>>>->>++++>+>+>>->->++>++>++>++++>+>++>->++>++++>+>+>++>++>->->++>++>++++>+>+>>>>>->>->>++++>++>++>++++>>>>>->>>>>+++>->++++>->->->+++>>>+>+>+++>+>++++>>+++>->>>>>->>>++++>++>++>+>+++>->++++>>->->+++>+>+++>+>++++>>>+++>->++++>>->->++>++++>++>++++>>++[-[->>+[>]++[<]<]>>+[>]<--[++>++++>]+[<]<<++]>>>[>]++++>++++[--[+>+>++++<<[-->>--<<[->-<[--->>+<<[+>+++<[+>>++<<]]]]]]>+++[>+++++++++++++++<-]>--.<<<]";
        let commands = BfParser::parse(commands_literally);
        machine.run(&commands).unwrap();

        let mut result = Box::new(vec![0; 255]);
        result.write(commands_literally.as_bytes()).unwrap();

        assert_eq!(output, result);
    }

    #[test]
    fn ascii_table() {
        let mut memory = vec![0; 255];
        let mut output = Box::new(vec![0; 255]);
        let input_supply = vec![];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let commands = BfParser::parse(".+[.+]");
        machine.run(&commands).unwrap();

        let mut result = Box::new(vec![0; 255]);
        for i in 0..256 {
            result.write(&[i as u8]).unwrap();
        }

        assert_eq!(output, result);
    }

    #[test]
    fn unclosed_loop() {
        let mut memory = vec![0; 255];
        let mut output = Box::new(vec![0; 255]);
        let input_supply = vec![];
        let mut input = Box::new(Cursor::new(&input_supply));
        let mut machine = BfMachine::new(&mut memory, &mut output, &mut input);

        let commands = BfParser::parse("[]]");
        let error = machine.run(&commands).unwrap_err();

        assert!(error.is::<BfRuntimeError>());
    }
}
