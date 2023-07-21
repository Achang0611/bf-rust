#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BfToken {
    NotCommand(char),
    Increment(u8),
    Decrement(u8),
    CursorLeft(usize),
    CursorRight(usize),
    LoopStart,
    LoopEnd,
    PrintChar,
    InputChar,
}
