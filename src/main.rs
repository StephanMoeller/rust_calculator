fn main() {
    _ = calculate("1 + 3 * (3-1) / 2").unwrap();
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Add,
    Subtract,
    Multiply,
    Divide,
    BeginParenthesis,
    EndParenthesis,
}

#[derive(Debug, PartialEq)]
enum CalculatorError {
    InvalidCharacter(char),
    DivideByZero,
}

fn calculate(input: &str) -> Result<i32, CalculatorError> {
    let tokens = tokenize(input)?;
    let root_node = build_tree(tokens)?;
    let result = eval(root_node);
    return result;
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalculatorError> {
    let mut tokens = vec![];
    let mut current_number: Option<i32> = None;
    for char in input.chars() {
        match char {
            '0'..='9' => current_number = Some(current_number.unwrap_or(0) * 10 + char as i32 - '0' as i32),
            _ => {
                if let Some(current_number) = current_number {
                    tokens.push(Token::Number(current_number));
                }
                current_number = None;

                match char {
                    '+' => tokens.push(Token::Add),
                    '-' => tokens.push(Token::Subtract),
                    '*' => tokens.push(Token::Multiply),
                    '/' => tokens.push(Token::Divide),
                    '(' => tokens.push(Token::BeginParenthesis),
                    ')' => tokens.push(Token::EndParenthesis),
                    ' ' => {}
                    _ => return Err(CalculatorError::InvalidCharacter(char)),
                }
            }
        }
    }
    if let Some(current_number) = current_number {
        tokens.push(Token::Number(current_number));
    }
    return Ok(tokens);
}

fn build_tree(tokens: Vec<Token>) -> Result<EvalutationNode, CalculatorError>
{
    return Result::Ok(EvalutationNode::Number(21));
}

#[derive(Debug, PartialEq)]
enum EvalutationNode {
    Number(i32),
    Complex(Box<EvalutationNode>, Token, Box<EvalutationNode>),
}


fn eval(node: EvalutationNode) -> Result<i32, CalculatorError> {
    match node {
        EvalutationNode::Number(val) => Result::Ok(val),
        EvalutationNode::Complex(left, operator, right) => {
            let left_value = eval(*left)?;
            let right_value = eval(*right)?;
            match operator {
                Token::Add => Result::Ok(left_value + right_value),
                Token::Subtract => Result::Ok(left_value - right_value),
                Token::Multiply => Result::Ok(left_value * right_value),
                Token::Divide => {
                    if right_value == 0 {
                        return Result::Err(CalculatorError::DivideByZero);
                    }
                    return Result::Ok(left_value / right_value);
                }
                _ => panic!("Unexpected token here!")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn tokenize_numbers_in_beginning_and_end_test() {
        let tokens = tokenize("1 + 232*32-5/2 + 21").unwrap();

        println!("{:?}", tokens);
        assert_eq!(11, tokens.len());
        assert_eq!(Token::Number(1), tokens[0]);
        assert_eq!(Token::Add, tokens[1]);
        assert_eq!(Token::Number(232), tokens[2]);
        assert_eq!(Token::Multiply, tokens[3]);
        assert_eq!(Token::Number(32), tokens[4]);
        assert_eq!(Token::Subtract, tokens[5]);
        assert_eq!(Token::Number(5), tokens[6]);
        assert_eq!(Token::Divide, tokens[7]);
        assert_eq!(Token::Number(2), tokens[8]);
        assert_eq!(Token::Add, tokens[9]);
        assert_eq!(Token::Number(21), tokens[10]);
    }

    #[test]
    fn tokenize_non_numbers_in_beginning_and_end_test() {
        let tokens = tokenize("(1 + 232*(32)-").unwrap();

        println!("{:?}", tokens);
        assert_eq!(9, tokens.len());
        assert_eq!(Token::BeginParenthesis, tokens[0]);
        assert_eq!(Token::Number(1), tokens[1]);
        assert_eq!(Token::Add, tokens[2]);
        assert_eq!(Token::Number(232), tokens[3]);
        assert_eq!(Token::Multiply, tokens[4]);
        assert_eq!(Token::BeginParenthesis, tokens[5]);
        assert_eq!(Token::Number(32), tokens[6]);
        assert_eq!(Token::EndParenthesis, tokens[7]);
        assert_eq!(Token::Subtract, tokens[8]);
    }

    #[test]
    fn tokenize_multiple_consecutive_numbers_test()
    {
        let tokens = tokenize("123 45 789 0 +").unwrap();

        assert_eq!(5, tokens.len());
        assert_eq!(Token::Number(123), tokens[0]);
        assert_eq!(Token::Number(45), tokens[1]);
        assert_eq!(Token::Number(789), tokens[2]);
        assert_eq!(Token::Number(0), tokens[3]);
        assert_eq!(Token::Add, tokens[4]);
    }

    #[test]
    fn tokenize_all_digits_test() {
        let tokens = tokenize("1234567890").unwrap();

        assert_eq!(1, tokens.len());
        assert_eq!(Token::Number(1234567890), tokens[0]);
    }

    #[test]
    fn tokenize_invalid_char_expect_error_test() {
        let result = tokenize("1 +M 232*32-  21");
        assert_eq!(Result::Err(CalculatorError::InvalidCharacter('M')), result);
    }

    #[test]
    fn eval_single_number_test() {
        let root = EvalutationNode::Number(28);
        let result = eval(root);
        assert_eq!(28, result.unwrap());
    }

    #[test]
    fn eval_simple_add_test() {
        let root = EvalutationNode::Complex(Box::from(EvalutationNode::Number(12)), Token::Add, Box::from(EvalutationNode::Number(32)));
        let result = eval(root);
        assert_eq!(12 + 32, result.unwrap());
    }

    #[test]
    fn eval_simple_subtract_test() {
        let root = EvalutationNode::Complex(Box::from(EvalutationNode::Number(12)), Token::Subtract, Box::from(EvalutationNode::Number(32)));
        let result = eval(root);
        assert_eq!(12 - 32, result.unwrap());
    }

    #[test]
    fn eval_simple_multiply_test() {
        let root = EvalutationNode::Complex(Box::from(EvalutationNode::Number(12)), Token::Multiply, Box::from(EvalutationNode::Number(32)));
        let result = eval(root);
        assert_eq!(12 * 32, result.unwrap());
    }

    #[test]
    fn eval_simple_divide_test() {
        let root = EvalutationNode::Complex(Box::from(EvalutationNode::Number(12)), Token::Divide, Box::from(EvalutationNode::Number(32)));
        let result = eval(root);
        assert_eq!(12 / 32, result.unwrap());
    }

    #[test]
    fn eval_complex_case_test() {
        let root = EvalutationNode::Complex(
            Box::from(EvalutationNode::Complex(
                Box::from(EvalutationNode::Number(21)),
                Token::Multiply,
                Box::from(EvalutationNode::Number(-12)),
            )),
            Token::Divide,
            Box::from(EvalutationNode::Number(4)));
        let result = eval(root);
        assert_eq!((21 * -12) / 4, result.unwrap());
    }
}
