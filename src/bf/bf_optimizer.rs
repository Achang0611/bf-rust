pub struct BfCodeOptimizer;

impl BfCodeOptimizer {
    pub fn optimize(code: &str) -> String {
        let code = Self::remove_not_command(code);
        let code = Self::remove_unnecessary_relative_operate(&code);
        code
    }

    fn remove_not_command(code: &str) -> String {
        code.chars()
            .filter(|c| matches!(c, '+' | '-' | ',' | '.' | '[' | ']' | '<' | '>'))
            .collect::<String>()
    }

    fn remove_unnecessary_relative_operate(code: &str) -> String {
        let mut result = String::new();

        for ch in code.chars() {
            let last_char = result.chars().last().unwrap_or('\0');
            if ch == '+' && last_char == '-'
                || ch == '-' && last_char == '+'
                || ch == '>' && last_char == '<'
                || ch == '<' && last_char == '>'
            {
                result.pop();
                continue;
            }

            result.push(ch);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::bf::bf_optimizer::BfCodeOptimizer;

    #[test]
    fn clear_not_command() {
        let code = BfCodeOptimizer::optimize("the quick brown fox jumps over the lazy dog-[],.<+>");
        assert_eq!(code, "-[],.<+>");
    }

    #[test]
    fn unnecessary_relative_operate() {
        let code = BfCodeOptimizer::optimize(">><<+");
        assert_eq!(code, "+".to_string());
        let code = BfCodeOptimizer::optimize(">><<><<>>><<><<>>");
        assert_eq!(code, ">".to_string());
        let code = BfCodeOptimizer::optimize(">>+<<+><+<>+>><+<>+<<+>>");
        assert_eq!(code, ">>+<<+++>++<<+>>".to_string());
        let code = BfCodeOptimizer::optimize(">>++--<<");
        assert_eq!(code, "".to_string());
        let code = BfCodeOptimizer::optimize(">>+++--<<");
        assert_eq!(code, ">>+<<".to_string());
    }
}
