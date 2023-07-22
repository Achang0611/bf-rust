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
        let mut tokens = vec![];
        let mut sum: i32 = 0;
        let mut cursor_move: i32 = 0;

        for ch in code.chars() {
            if ch != '+' && ch != '-' && sum != 0 {
                if sum > 0 {
                    tokens.push(BfToken::Increment(sum as u8));
                } else if sum < 0 {
                    tokens.push(BfToken::Decrement(-sum as u8))
                }
                sum = 0;
            }
            if ch != '>' && ch != '<' && cursor_move != 0 {
                if cursor_move > 0 {
                    tokens.push(BfToken::CursorRight(cursor_move as usize));
                } else if cursor_move < 0 {
                    tokens.push(BfToken::CursorLeft(-cursor_move as usize))
                }
                cursor_move = 0;
            }

            match ch {
                '+' => sum += 1,
                '-' => sum -= 1,
                '<' => cursor_move -= 1,
                '>' => cursor_move += 1,
                '[' => tokens.push(BfToken::LoopStart),
                ']' => tokens.push(BfToken::LoopEnd),
                ',' => tokens.push(BfToken::InputChar),
                '.' => tokens.push(BfToken::PrintChar),
                _ => tokens.push(BfToken::NotCommand(ch)),
            }
        }

        if sum != 0 {
            if sum > 0 {
                tokens.push(BfToken::Increment(sum as u8));
            } else if sum < 0 {
                tokens.push(BfToken::Decrement(sum.abs() as u8))
            }
        }

        if cursor_move != 0 {
            if cursor_move > 0 {
                tokens.push(BfToken::CursorRight(cursor_move as usize));
            } else if cursor_move < 0 {
                tokens.push(BfToken::CursorLeft(cursor_move.abs() as usize))
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

    #[test]
    fn parse_compress() {
        let tokens = BfParser::parse_compress("+++++--->>>><<");
        assert_eq!(&tokens, &[BfToken::Increment(2), BfToken::CursorRight(2),]);

        let tokens = BfParser::parse_compress("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.",);
        assert_eq!(
            &tokens,
            &[
                BfToken::Increment(10),
                BfToken::LoopStart,
                BfToken::CursorRight(1),
                BfToken::Increment(7),
                BfToken::CursorRight(1),
                BfToken::Increment(10),
                BfToken::CursorRight(1),
                BfToken::Increment(3),
                BfToken::CursorRight(1),
                BfToken::Increment(1),
                BfToken::CursorLeft(4),
                BfToken::Decrement(1),
                BfToken::LoopEnd,
                BfToken::CursorRight(1),
                BfToken::Increment(2),
                BfToken::PrintChar,
                BfToken::CursorRight(1),
                BfToken::Increment(1),
                BfToken::PrintChar,
                BfToken::Increment(7),
                BfToken::PrintChar,
                BfToken::PrintChar,
                BfToken::Increment(3),
                BfToken::PrintChar,
                BfToken::CursorRight(1),
                BfToken::Increment(2),
                BfToken::PrintChar,
                BfToken::CursorLeft(2),
                BfToken::Increment(15),
                BfToken::PrintChar,
                BfToken::CursorRight(1),
                BfToken::PrintChar,
                BfToken::Increment(3),
                BfToken::PrintChar,
                BfToken::Decrement(6),
                BfToken::PrintChar,
                BfToken::Decrement(8),
                BfToken::PrintChar,
                BfToken::CursorRight(1),
                BfToken::Increment(1),
                BfToken::PrintChar,
                BfToken::CursorRight(1),
                BfToken::PrintChar
            ]
        );
    }
}
