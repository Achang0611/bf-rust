pub struct BfParser;

use super::bf_token::BfToken;

impl BfParser {
    pub fn parse(code: &str) -> Vec<BfToken> {
        let mut tokens = vec![];

        for command in code.chars() {
            match command {
                '+' => tokens.push(BfToken::Increament),
                '-' => tokens.push(BfToken::Decreament),
                '<' => tokens.push(BfToken::CursorLeft),
                '>' => tokens.push(BfToken::CursorRight),
                ',' => tokens.push(BfToken::InputChar),
                '.' => tokens.push(BfToken::PrintChar),
                '[' => tokens.push(BfToken::LoopStart),
                ']' => tokens.push(BfToken::LoopEnd),
                _ => tokens.push(BfToken::NotCommand),
            }
        }

        tokens
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
                BfToken::NotCommand,
                BfToken::Increament,
                BfToken::Decreament,
                BfToken::LoopStart,
                BfToken::LoopEnd,
                BfToken::InputChar,
                BfToken::PrintChar,
                BfToken::CursorLeft,
                BfToken::CursorRight,
            ]
        );
    }
}
