#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BfToken {
    NotCommand,
    Increament,
    Decreament,
    CursorLeft,
    CursorRight,
    LoopStart,
    LoopEnd,
    PrintChar,
    InputChar,
}
