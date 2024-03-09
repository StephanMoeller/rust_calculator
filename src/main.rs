use std::collections::{HashMap, HashSet};

fn main() {
    let result = calculate("1 + 3 * (3-1) / 2").unwrap();
    println!("Result: {}", result);
}

fn calculate(input: &str) -> Result<i32, CalculatorError> {
    let tokens = tokenize(input)?;
    validate_token_sequence(&tokens)?;
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
                    '+' => tokens.push(Token::Plus),
                    '-' => tokens.push(Token::Minus),
                    '*' => tokens.push(Token::Star),
                    '/' => tokens.push(Token::Slash),
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

fn validate_token_sequence(tokens: &Vec<Token>) -> Result<(), CalculatorError>
{
    if tokens.len() == 0 {
        return Result::Err(CalculatorError::EmptyStatement);
    }

    // The rules for which tokens can follow each other are:
    // format: [token] > [valid followers]
    // START > AnyNumber, BeginGroup
    // BeginGroup > AnyNumber, BeginGroup
    // AnyNumber > AnyOperator, EndGroup
    // AnyOperator > AnyNumber, BeginGroup
    // EndGroup > EndGroup, AnyOperator
    let followers_by_token: HashMap<TokenCategory, HashSet<TokenCategory>> = HashMap::from([
        (TokenCategory::AnyNumber, HashSet::from([TokenCategory::AnyOperator, TokenCategory::EndGroup])),
        (TokenCategory::AnyOperator, HashSet::from([TokenCategory::AnyNumber, TokenCategory::BeginGroup])),
        (TokenCategory::EndGroup, HashSet::from([TokenCategory::EndGroup, TokenCategory::AnyOperator])),
        (TokenCategory::BeginGroup, HashSet::from([TokenCategory::AnyNumber, TokenCategory::BeginGroup]))
    ]);

    let mut prev_cat = TokenCategory::BeginGroup;

    // Validate sequence
    for i in 0..tokens.len() {
        let current_cat = get_category(&tokens[i]);

        if !followers_by_token[&prev_cat].contains(&current_cat)
        {
            return Result::Err(CalculatorError::InvalidTokenSequence(prev_cat, tokens[i].clone()));
        }

        prev_cat = current_cat;
    }

    return Result::Ok(());
}

fn build_tree(_tokens: Vec<Token>) -> Result<EvaluationNode, CalculatorError>
{
    return Ok(EvaluationNode::Number(21));
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum TokenCategory
{
    AnyNumber,
    AnyOperator,
    BeginGroup,
    EndGroup,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    BeginParenthesis,
    EndParenthesis,
}

fn get_category(token: &Token) -> TokenCategory {
    match token {
        Token::Number(_) => TokenCategory::AnyNumber,
        Token::Plus => TokenCategory::AnyOperator,
        Token::Minus => TokenCategory::AnyOperator,
        Token::Star => TokenCategory::AnyOperator,
        Token::Slash => TokenCategory::AnyOperator,
        Token::BeginParenthesis => TokenCategory::BeginGroup,
        Token::EndParenthesis => TokenCategory::EndGroup
    }
}

#[derive(Debug, PartialEq)]
enum CalculatorError {
    InvalidCharacter(char),
    DivideByZero,
    EmptyStatement,
    InvalidTokenSequence(TokenCategory, Token),
}

#[derive(Debug, PartialEq)]
enum EvaluationNode {
    Number(i32),
    Complex(Box<EvaluationNode>, Operator, Box<EvaluationNode>),
}


#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn eval(node: EvaluationNode) -> Result<i32, CalculatorError> {
    match node {
        EvaluationNode::Number(val) => Ok(val),
        EvaluationNode::Complex(left, operator, right) => {
            let left_value = eval(*left)?;
            let right_value = eval(*right)?;
            match operator {
                Operator::Add => Result::Ok(left_value + right_value),
                Operator::Subtract => Result::Ok(left_value - right_value),
                Operator::Multiply => Result::Ok(left_value * right_value),
                Operator::Divide => {
                    if right_value == 0 {
                        return Result::Err(CalculatorError::DivideByZero);
                    }
                    return Result::Ok(left_value / right_value);
                }
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
        assert_eq!(Token::Plus, tokens[1]);
        assert_eq!(Token::Number(232), tokens[2]);
        assert_eq!(Token::Star, tokens[3]);
        assert_eq!(Token::Number(32), tokens[4]);
        assert_eq!(Token::Minus, tokens[5]);
        assert_eq!(Token::Number(5), tokens[6]);
        assert_eq!(Token::Slash, tokens[7]);
        assert_eq!(Token::Number(2), tokens[8]);
        assert_eq!(Token::Plus, tokens[9]);
        assert_eq!(Token::Number(21), tokens[10]);
    }

    #[test]
    fn tokenize_non_numbers_in_beginning_and_end_test() {
        let tokens = tokenize("(1 + 232*(32)-").unwrap();

        println!("{:?}", tokens);
        assert_eq!(9, tokens.len());
        assert_eq!(Token::BeginParenthesis, tokens[0]);
        assert_eq!(Token::Number(1), tokens[1]);
        assert_eq!(Token::Plus, tokens[2]);
        assert_eq!(Token::Number(232), tokens[3]);
        assert_eq!(Token::Star, tokens[4]);
        assert_eq!(Token::BeginParenthesis, tokens[5]);
        assert_eq!(Token::Number(32), tokens[6]);
        assert_eq!(Token::EndParenthesis, tokens[7]);
        assert_eq!(Token::Minus, tokens[8]);
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
        assert_eq!(Token::Plus, tokens[4]);
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
        let root = EvaluationNode::Number(28);
        let result = eval(root);
        assert_eq!(28, result.unwrap());
    }

    #[test]
    fn eval_simple_add_test() {
        let root = EvaluationNode::Complex(Box::from(EvaluationNode::Number(12)), Operator::Add, Box::from(EvaluationNode::Number(32)));
        let result = eval(root);
        assert_eq!(12 + 32, result.unwrap());
    }

    #[test]
    fn eval_simple_subtract_test() {
        let root = EvaluationNode::Complex(Box::from(EvaluationNode::Number(12)), Operator::Subtract, Box::from(EvaluationNode::Number(32)));
        let result = eval(root);
        assert_eq!(12 - 32, result.unwrap());
    }

    #[test]
    fn eval_simple_multiply_test() {
        let root = EvaluationNode::Complex(Box::from(EvaluationNode::Number(12)), Operator::Multiply, Box::from(EvaluationNode::Number(32)));
        let result = eval(root);
        assert_eq!(12 * 32, result.unwrap());
    }

    #[test]
    fn eval_simple_divide_test() {
        let root = EvaluationNode::Complex(Box::from(EvaluationNode::Number(12)), Operator::Divide, Box::from(EvaluationNode::Number(32)));
        let result = eval(root);
        assert_eq!(12 / 32, result.unwrap());
    }

    #[test]
    fn eval_complex_case_test() {
        let root = EvaluationNode::Complex(
            Box::from(EvaluationNode::Complex(
                Box::from(EvaluationNode::Number(21)),
                Operator::Multiply,
                Box::from(EvaluationNode::Number(-12)),
            )),
            Operator::Divide,
            Box::from(EvaluationNode::Number(4)));
        let result = eval(root);
        assert_eq!((21 * -12) / 4, result.unwrap());
    }

    #[test]
    fn validate_token_sequence_invalid_end_para_test()
    {
        let tokens = tokenize(")").unwrap();
        let result = validate_token_sequence(&tokens);
        assert_eq!(result, Err(CalculatorError::InvalidTokenSequence(TokenCategory::BeginGroup, Token::EndParenthesis)));
    }
}
