use std::{error::Error, fmt::Display};

use super::bf_token::BfToken;

pub struct BfParser;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BfParserError {
    LoopNotClosed(usize),
}

impl BfParser {
    pub fn parse(code: &str) -> Result<Vec<BfToken>, BfParserError> {
        let mut tokens = vec![];

        for ch in code.chars() {
            match ch {
                '+' => tokens.push(BfToken::Increment(1)),
                '-' => tokens.push(BfToken::Decrement(1)),
                '<' => tokens.push(BfToken::CursorLeft(1)),
                '>' => tokens.push(BfToken::CursorRight(1)),
                '[' => tokens.push(BfToken::LoopStart(0)),
                ']' => tokens.push(BfToken::LoopEnd(0)),
                ',' => tokens.push(BfToken::InputChar),
                '.' => tokens.push(BfToken::PrintChar),
                _ => tokens.push(BfToken::NotCommand(ch)),
            }
        }

        Self::loop_matching(&mut tokens)?;

        Ok(tokens)
    }

    pub fn parse_compress(code: &str) -> Result<Vec<BfToken>, BfParserError> {
        let uncompress_tokens = Self::parse(code)?;
        let mut tokens = vec![];
        let mut sum = 0i32;
        let mut cursor_move = 0i32;

        for token in uncompress_tokens.into_iter() {
            if !matches!(token, BfToken::Increment(_) | BfToken::Decrement(_)) && sum != 0 {
                if sum > 0 {
                    tokens.push(BfToken::Increment(sum as u8));
                } else if sum < 0 {
                    tokens.push(BfToken::Decrement(-sum as u8))
                }
                sum = 0;
            }
            if !matches!(token, BfToken::CursorLeft(_) | BfToken::CursorRight(_))
                && cursor_move != 0
            {
                if cursor_move > 0 {
                    tokens.push(BfToken::CursorRight(cursor_move as usize));
                } else if cursor_move < 0 {
                    tokens.push(BfToken::CursorLeft(-cursor_move as usize))
                }
                cursor_move = 0;
            }

            match token {
                BfToken::Increment(_) => sum += 1,
                BfToken::Decrement(_) => sum -= 1,
                BfToken::CursorLeft(_) => cursor_move -= 1,
                BfToken::CursorRight(_) => cursor_move += 1,
                _ => tokens.push(token),
            }
        }

        if sum != 0 {
            if sum > 0 {
                tokens.push(BfToken::Increment(sum as u8));
            } else if sum < 0 {
                tokens.push(BfToken::Decrement(-sum as u8))
            }
        }

        if cursor_move != 0 {
            if cursor_move > 0 {
                tokens.push(BfToken::CursorRight(cursor_move as usize));
            } else if cursor_move < 0 {
                tokens.push(BfToken::CursorLeft(-cursor_move as usize))
            }
        }

        Self::loop_matching(&mut tokens)?;

        Ok(tokens)
    }

    fn loop_matching(tokens: &mut [BfToken]) -> Result<(), BfParserError> {
        let mut loop_record = vec![];

        for index in 0..tokens.len() {
            match tokens[index] {
                BfToken::LoopStart(_) => {
                    loop_record.push(index);
                }
                BfToken::LoopEnd(_) => {
                    let match_start = loop_record
                        .pop()
                        .ok_or(BfParserError::LoopNotClosed(index))?;
                    tokens[match_start] = BfToken::LoopStart(index);
                    tokens[index] = BfToken::LoopEnd(match_start);
                }
                _ => {}
            }
        }

        if !loop_record.is_empty() {
            return Err(BfParserError::LoopNotClosed(loop_record.pop().unwrap()));
        }

        Ok(())
    }
}

impl Display for BfParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::LoopNotClosed(index) => {
                format!("The error occurred at index {index} due to an unclosed loop.")
            }
        };
        write!(f, "{message}")
    }
}

impl Error for BfParserError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_commands_parse() {
        let tokens = BfParser::parse("a+-[],.<>").unwrap();

        assert_eq!(
            &tokens,
            &[
                BfToken::NotCommand('a'),
                BfToken::Increment(1),
                BfToken::Decrement(1),
                BfToken::LoopStart(4),
                BfToken::LoopEnd(3),
                BfToken::InputChar,
                BfToken::PrintChar,
                BfToken::CursorLeft(1),
                BfToken::CursorRight(1),
            ]
        );
    }

    #[test]
    fn parse_compress() {
        let tokens = BfParser::parse_compress("+++++--->>>><<").unwrap();
        assert_eq!(&tokens, &[BfToken::Increment(2), BfToken::CursorRight(2),]);

        let tokens = BfParser::parse_compress("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.",).unwrap();
        assert_eq!(
            &tokens,
            &[
                BfToken::Increment(10),
                BfToken::LoopStart(12),
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
                BfToken::LoopEnd(1),
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

    #[test]
    fn unclosed_loop() {
        let tokens = BfParser::parse("[]]").unwrap_err();
        assert_eq!(tokens, BfParserError::LoopNotClosed(2));
        let tokens = BfParser::parse("[[]").unwrap_err();
        assert_eq!(tokens, BfParserError::LoopNotClosed(0));
        let tokens = BfParser::parse("[[[]").unwrap_err();
        assert_eq!(tokens, BfParserError::LoopNotClosed(1));
    }
}
