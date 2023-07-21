use super::bf_token::BfToken;

pub struct BfParser;

impl BfParser {
    pub fn parse(code: &str) -> Vec<BfToken> {
        let mut tokens = vec![];

        for command in code.chars() {
            match command {
                '+' => tokens.push(BfToken::Increment(1)),
                '-' => tokens.push(BfToken::Decrement(1)),
                '<' => tokens.push(BfToken::CursorLeft(1)),
                '>' => tokens.push(BfToken::CursorRight(1)),
                '[' => tokens.push(BfToken::LoopStart),
                ']' => tokens.push(BfToken::LoopEnd),
                ',' => tokens.push(BfToken::InputChar),
                '.' => tokens.push(BfToken::PrintChar),
                _ => tokens.push(BfToken::NotCommand(command)),
            }
        }

        tokens
    }

    pub fn parse_compress(code: &str) -> Vec<BfToken> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_commands_parse() {
        let tokens = BfParser::parse("a+-[],.<>");

        assert_eq!(
            &tokens,
            &[
                BfToken::NotCommand('a'),
                BfToken::Increment(1),
                BfToken::Decrement(1),
                BfToken::LoopStart,
                BfToken::LoopEnd,
                BfToken::InputChar,
                BfToken::PrintChar,
                BfToken::CursorLeft(1),
                BfToken::CursorRight(1),
            ]
        );
    }
}
