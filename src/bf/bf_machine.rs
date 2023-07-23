use std::{
    error::Error,
    fmt::{Debug, Display},
    io::{stdin, stdout, Read, Stdin, Stdout, Write},
};

use super::bf_token::BfToken;

pub struct BfMachine<R, W>
where
    R: Read,
    W: Write,
{
    cursor: usize,
    memory: Vec<u8>,
    input: R,
    output: W,
}

pub struct BfState {
    commands: Vec<BfToken>,
    program_counter: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BfRuntimeError {
    LoopNotClosed(usize),
}

impl<R, W> BfMachine<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(memory_size: usize, input: R, output: W) -> Self {
        assert!(memory_size > 0);

        let memory = vec![0; memory_size];
        Self {
            cursor: 0,
            memory,
            input,
            output,
        }
    }

    pub fn run(&mut self, commands: &[BfToken]) -> Result<(), Box<dyn Error>> {
        let mut state = BfState {
            commands: commands.to_vec(),
            program_counter: 0,
        };

        while state.program_counter < state.commands.len() {
            match state.commands[state.program_counter] {
                BfToken::NotCommand(_) => {}
                BfToken::Increment(val) => {
                    self.memory[self.cursor] = self.memory[self.cursor].wrapping_add(val);
                }
                BfToken::Decrement(val) => {
                    self.memory[self.cursor] = self.memory[self.cursor].wrapping_sub(val);
                }
                BfToken::CursorLeft(val) => {
                    self.cursor = Self::wrapped_cursor(self.cursor, true, val, self.memory.len());
                }
                BfToken::CursorRight(val) => {
                    self.cursor = Self::wrapped_cursor(self.cursor, false, val, self.memory.len());
                }
                BfToken::LoopStart(to_end) => {
                    if self.memory[self.cursor] == 0 {
                        state.program_counter = to_end;
                    }
                }
                BfToken::LoopEnd(to_start) => {
                    if self.memory[self.cursor] != 0 {
                        state.program_counter = to_start;
                    }
                }
                BfToken::PrintChar => {
                    self.output.write(&vec![self.memory[self.cursor]])?;
                }
                BfToken::InputChar => {
                    let mut input = [0; 1];
                    self.input.read_exact(&mut input)?;
                    self.memory[self.cursor] = input[0];
                }
            }

            state.program_counter += 1;
        }

        Ok(())
    }

    fn wrapped_cursor(cursor: usize, sign: bool, offset: usize, bound: usize) -> usize {
        if sign {
            if offset > cursor {
                (bound - (offset % bound) + cursor) % bound
            } else {
                (cursor - offset) % bound
            }
        } else {
            (cursor + offset) % bound
        }
    }
}

impl<R, W> Debug for BfMachine<R, W>
where
    R: Read + Debug,
    W: Write + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BfMachine")
            .field("cursor", &self.cursor)
            .field("memory", &self.memory)
            .field("input", &self.input)
            .field("output", &self.output)
            .finish()
    }
}

impl Default for BfMachine<Stdin, Stdout> {
    fn default() -> Self {
        Self::new(30_000, stdin(), stdout())
    }
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

    fn create_test_machine(input: &[u8]) -> BfMachine<std::io::Cursor<Vec<u8>>, Vec<u8>> {
        BfMachine::new(30000, Cursor::new(input.to_owned()), vec![0; 30000])
    }

    #[test]
    fn hello_world() {
        let mut machine = create_test_machine(&[]);

        let commands = BfParser::parse(
            "++++++++++[>+++++++>++++++++++>+++>+<<<<-]
    >++.>+.+++++++..+++.>++.<<+++++++++++++++.
    >.+++.------.--------.>+.>.",
        )
        .unwrap();
        machine.run(&commands).unwrap();

        let mut result = vec![0; 30000];
        result.write("Hello World!\n".as_bytes()).unwrap();

        assert_eq!(machine.output, result);
    }

    #[test]
    fn fast_hello_world() {
        let mut machine = create_test_machine(&[]);

        let commands = BfParser::parse_compress(
            "++++++++++[>+++++++>++++++++++>+++>+<<<<-]
    >++.>+.+++++++..+++.>++.<<+++++++++++++++.
    >.+++.------.--------.>+.>.",
        )
        .unwrap();
        machine.run(&commands).unwrap();

        let mut result = vec![0; 30000];
        result.write("Hello World!\n".as_bytes()).unwrap();

        assert_eq!(machine.output, result);
    }

    #[test]
    fn clear_cursor_memory() {
        let mut machine = create_test_machine(&[]);
        machine.memory[0] = 10;

        let commands = BfParser::parse("[-]").unwrap();

        assert_eq!(machine.memory[0], 10);

        machine.run(&commands).unwrap();

        assert_eq!(machine.memory[0], 0);
    }

    #[test]
    fn input_and_output() {
        let mut machine = create_test_machine(&['t' as u8]);

        let commands = BfParser::parse(",.").unwrap();
        machine.run(&commands).unwrap();

        let mut result = vec![0; 30000];
        result.write("t".as_bytes()).unwrap();

        assert_eq!(machine.output, result);
    }

    #[test]
    fn run_batch_commands() {
        let mut machine = create_test_machine(&[]);

        let assign_cell_0_to_10 = BfParser::parse("[-]++++++++++").unwrap();
        let move_cell_0_to_cell_1 = BfParser::parse("[>+ <-]").unwrap();
        let clear_cell_1 = BfParser::parse(">[-]").unwrap();

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
        let mut machine = create_test_machine(&[]);

        let commands = BfParser::parse(
            "++++>++++>[-]>[-]>[-]<<<<[->[->+>+<<]>>[-<<+>>]>+<<<<]>>>>[-<<<<+>>>>]<<<<",
        )
        .unwrap();
        machine.run(&commands).unwrap();

        assert_eq!(machine.memory[0], 4);
        assert_eq!(machine.memory[1], 4);
        assert_eq!(machine.memory[2], 16);
    }

    #[test]
    fn pi() {
        let mut machine = create_test_machine(&[]);

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
        )
        .unwrap();
        machine.run(&commands).unwrap();

        let mut result = vec![0; 30000];
        result.write("3.14070455282885\n".as_bytes()).unwrap();

        assert_eq!(machine.output, result);
    }

    #[test]
    fn fast_pi() {
        let mut machine = create_test_machine(&[]);

        let commands = BfParser::parse_compress(
            ">+++++++++++++++[<+>>>>>>>>++++++++++<<<<<<<-]>+++++[<+++++++++>-]+>>>>>>+[<<+++
            [>>[-<]<[>]<-]>>[>+>]<[<]>]>[[->>>>+<<<<]>>>+++>-]<[<<<<]<<<<<<<<+[->>>>>>>>>>>>
            [<+[->>>>+<<<<]>>>>>]<<<<[>>>>>[<<<<+>>>>-]<<<<<-[<<++++++++++>>-]>>>[<<[<+<<+>>
            >-]<[>+<-]<++<<+>>>>>>-]<<[-]<<-<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>+<<<-[>>+<<
            -]<]<<<<+>>>>>>>>[-]>[<<<+>>>-]<<++++++++++<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>
            +>[<<+<+>>>-]<<<<+<+>>[-[-[-[-[-[-[-[-[-<->[-<+<->>]]]]]]]]]]<[+++++[<<<++++++++
            <++++++++>>>>-]<<<<+<->>>>[>+<<<+++++++++<->>>-]<<<<<[>>+<<-]+<[->-<]>[>>.<<<<[+
            .[-]]>>-]>[>>.<<-]>[-]>[-]>>>[>>[<<<<<<<<+>>>>>>>>-]<<-]]>>[-]<<<[-]<<<<<<<<]+++
            +++++++.",
        )
        .unwrap();
        machine.run(&commands).unwrap();

        let mut result = vec![0; 30000];
        result.write("3.14070455282885\n".as_bytes()).unwrap();

        assert_eq!(machine.output, result);
    }

    #[test]
    fn quine() {
        let mut machine = create_test_machine(&[]);

        let commands_literally =
            "-->+++>+>+>+>+++++>++>++>->+++>++>+>>>>>>>>>>>>>>>>->++++>>>>->+++>+++>+++>+++>+++>+++>+>+>>>->->>++++>+>>>>->>++++>+>+>>->->++>++>++>++++>+>++>->++>++++>+>+>++>++>->->++>++>++++>+>+>>>>>->>->>++++>++>++>++++>>>>>->>>>>+++>->++++>->->->+++>>>+>+>+++>+>++++>>+++>->>>>>->>>++++>++>++>+>+++>->++++>>->->+++>+>+++>+>++++>>>+++>->++++>>->->++>++++>++>++++>>++[-[->>+[>]++[<]<]>>+[>]<--[++>++++>]+[<]<<++]>>>[>]++++>++++[--[+>+>++++<<[-->>--<<[->-<[--->>+<<[+>+++<[+>>++<<]]]]]]>+++[>+++++++++++++++<-]>--.<<<]";
        let commands = BfParser::parse(commands_literally).unwrap();
        machine.run(&commands).unwrap();

        let mut result = vec![0; 30000];
        result.write(commands_literally.as_bytes()).unwrap();

        assert_eq!(machine.output, result);
    }

    #[test]
    fn fast_quine() {
        let mut machine = create_test_machine(&[]);

        let commands_literally =
            "-->+++>+>+>+>+++++>++>++>->+++>++>+>>>>>>>>>>>>>>>>->++++>>>>->+++>+++>+++>+++>+++>+++>+>+>>>->->>++++>+>>>>->>++++>+>+>>->->++>++>++>++++>+>++>->++>++++>+>+>++>++>->->++>++>++++>+>+>>>>>->>->>++++>++>++>++++>>>>>->>>>>+++>->++++>->->->+++>>>+>+>+++>+>++++>>+++>->>>>>->>>++++>++>++>+>+++>->++++>>->->+++>+>+++>+>++++>>>+++>->++++>>->->++>++++>++>++++>>++[-[->>+[>]++[<]<]>>+[>]<--[++>++++>]+[<]<<++]>>>[>]++++>++++[--[+>+>++++<<[-->>--<<[->-<[--->>+<<[+>+++<[+>>++<<]]]]]]>+++[>+++++++++++++++<-]>--.<<<]";
        let commands = BfParser::parse_compress(commands_literally).unwrap();
        machine.run(&commands).unwrap();

        let mut result = vec![0; 30000];
        result.write(commands_literally.as_bytes()).unwrap();

        assert_eq!(machine.output, result);
    }

    #[test]
    fn ascii_table() {
        let mut machine = create_test_machine(&[]);

        let commands = BfParser::parse(".+[.+]").unwrap();
        machine.run(&commands).unwrap();

        let mut result = vec![0; 30000];
        for i in 0..256 {
            result.write(&[i as u8]).unwrap();
        }

        assert_eq!(machine.output, result);
    }

    #[test]
    fn memory_overflow_wrapped() {
        let mut machine = create_test_machine(&[]);

        let code = "<+>";
        let equal_code = format!("{}+>", ">".repeat(machine.memory.len() - 1));
        let overflow_code = ">".repeat(machine.memory.len() * 2);

        let commands = BfParser::parse(&code).unwrap();
        let equal_commands = BfParser::parse_compress(&equal_code).unwrap();
        let overflow_commands = BfParser::parse_compress(&overflow_code).unwrap();

        machine.run(&commands).unwrap();
        assert_eq!(machine.memory[machine.memory.len() - 1], 1);
        machine.run(&equal_commands).unwrap();
        assert_eq!(machine.memory[machine.memory.len() - 1], 2);
        machine.run(&overflow_commands).unwrap();
        assert_eq!(machine.cursor, 0);
    }
}
