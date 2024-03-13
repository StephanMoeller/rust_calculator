extern crate core;

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
                    '+' => tokens.push(Token::Operator(Operator::Add)),
                    '-' => tokens.push(Token::Operator(Operator::Subtract)),
                    '*' => tokens.push(Token::Operator(Operator::Multiply)),
                    '/' => tokens.push(Token::Operator(Operator::Divide)),
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

    let mut prev_token = Token::BeginParenthesis;

    // Validate sequence
    for i in 0..tokens.len() {
        let current_token = tokens[i].clone();
        let prev_copy = prev_token.clone();
        match prev_copy {
            Token::Number(_) => {
                match current_token {
                    Token::Number(_) => return Result::Err(CalculatorError::InvalidTokenSequence(prev_copy, current_token.clone())),
                    Token::Operator(_) => {}
                    Token::BeginParenthesis => return Result::Err(CalculatorError::InvalidTokenSequence(prev_copy, current_token.clone())),
                    Token::EndParenthesis => {}
                }
            }
            Token::Operator(_) => {
                match current_token {
                    Token::Number(_) => {}
                    Token::Operator(_) => return Result::Err(CalculatorError::InvalidTokenSequence(prev_copy, current_token.clone())),
                    Token::BeginParenthesis => {}
                    Token::EndParenthesis => return Result::Err(CalculatorError::InvalidTokenSequence(prev_copy, current_token.clone())),
                }
            }
            Token::BeginParenthesis => {
                match current_token {
                    Token::Number(_) => {}
                    Token::Operator(_) => return Result::Err(CalculatorError::InvalidTokenSequence(prev_copy, current_token.clone())),
                    Token::BeginParenthesis => {}
                    Token::EndParenthesis => return Result::Err(CalculatorError::InvalidTokenSequence(prev_copy, current_token.clone())),
                }
            }
            Token::EndParenthesis => {
                match current_token {
                    Token::Number(_) => return Result::Err(CalculatorError::InvalidTokenSequence(prev_copy, current_token.clone())),
                    Token::Operator(_) => {}
                    Token::BeginParenthesis => return Result::Err(CalculatorError::InvalidTokenSequence(prev_copy, current_token.clone())),
                    Token::EndParenthesis => {}
                }
            }
        }

        prev_token = current_token.clone();
    }

    return Result::Ok(());
}

fn build_tree(mut tokens: Vec<Token>) -> Result<EvaluationNode, CalculatorError>
{
    if tokens.len() == 0 {
        panic!("Empty token list not expected here!");
    }

    // Start out by letting first node represent the entire tree so far
    let first_single_node: EvaluationNode = match tokens.remove(0) {
        Token::Number(num) => EvaluationNode::Number(num),
        _ => panic!("Expected number at first position.")
    };

    let result = append_to_tree(first_single_node, tokens);
    match result {
        Result::Ok(final_tree) => Result::Ok(final_tree),
        Result::Err(err) => Result::Err(err)
    }
}

fn append_to_tree(tree: EvaluationNode, mut tokens: Vec<Token>) -> Result<EvaluationNode, CalculatorError>
{
    match tokens.len() {
        0 => {
            return Result::Ok(tree);
        }
        1 => panic!("Expected 0 or 2+ nodes here"),
        _ => {
            // Expect operator as first node
            match tokens.remove(0) {
                Token::Operator(op) => {
                    // Expect next node to be a number
                    match tokens.remove(0) {
                        Token::Number(num) => {
                            // Start out by adding next number in this way:
                            //         op
                            //      /      \
                            //     /        \
                            //  OLD TREE  new_node
                            let tree = EvaluationNode::Complex(Box::new(tree), op, Box::new(EvaluationNode::Number(num)));

                            // ...and now call recursively with this new tree and the remaining tokens
                            let tree = append_to_tree(tree, tokens).unwrap();
                            return Result::Ok(tree);
                        }
                        _ => panic!("Expected number here")
                    }
                }
                _ => panic!("Expected operator here")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i32),
    Operator(Operator),
    BeginParenthesis,
    EndParenthesis,
}

#[derive(Debug, PartialEq)]
enum CalculatorError {
    InvalidCharacter(char),
    DivideByZero,
    EmptyStatement,
    InvalidTokenSequence(Token, Token),
}

#[derive(Debug, PartialEq)]
enum EvaluationNode {
    Number(i32),
    Complex(Box<EvaluationNode>, Operator, Box<EvaluationNode>),
}


#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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
        assert_eq!(Token::Operator(Operator::Add), tokens[1]);
        assert_eq!(Token::Number(232), tokens[2]);
        assert_eq!(Token::Operator(Operator::Multiply), tokens[3]);
        assert_eq!(Token::Number(32), tokens[4]);
        assert_eq!(Token::Operator(Operator::Subtract), tokens[5]);
        assert_eq!(Token::Number(5), tokens[6]);
        assert_eq!(Token::Operator(Operator::Divide), tokens[7]);
        assert_eq!(Token::Number(2), tokens[8]);
        assert_eq!(Token::Operator(Operator::Add), tokens[9]);
        assert_eq!(Token::Number(21), tokens[10]);
    }

    #[test]
    fn tokenize_non_numbers_in_beginning_and_end_test() {
        let tokens = tokenize("(1 + 232*(32)-").unwrap();

        println!("{:?}", tokens);
        assert_eq!(9, tokens.len());
        assert_eq!(Token::BeginParenthesis, tokens[0]);
        assert_eq!(Token::Number(1), tokens[1]);
        assert_eq!(Token::Operator(Operator::Add), tokens[2]);
        assert_eq!(Token::Number(232), tokens[3]);
        assert_eq!(Token::Operator(Operator::Multiply), tokens[4]);
        assert_eq!(Token::BeginParenthesis, tokens[5]);
        assert_eq!(Token::Number(32), tokens[6]);
        assert_eq!(Token::EndParenthesis, tokens[7]);
        assert_eq!(Token::Operator(Operator::Subtract), tokens[8]);
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
        assert_eq!(Token::Operator(Operator::Add), tokens[4]);
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
    fn validate_token_sequence_beginning_of_phrases_test()
    {
        run_and_expect_error(")", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::EndParenthesis));
        run_and_expect_error("*", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::Operator(Operator::Multiply)));
        run_and_expect_error("/", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::Operator(Operator::Divide)));
        run_and_expect_error("+", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::Operator(Operator::Add)));
        run_and_expect_error("-", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::Operator(Operator::Subtract)));
        run_and_expect_error("  ) + 12342", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::EndParenthesis));
        run_and_expect_error("  * + 12342", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::Operator(Operator::Multiply)));
        run_and_expect_error("  / + 12342", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::Operator(Operator::Divide)));
        run_and_expect_error("  + + 12342", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::Operator(Operator::Add)));
        run_and_expect_error("  - + 12342", CalculatorError::InvalidTokenSequence(Token::BeginParenthesis, Token::Operator(Operator::Subtract)));
    }

    fn run_and_expect_error(phrase: &str, expected_error: CalculatorError)
    {
        let tokens = tokenize(phrase).unwrap();
        let result = validate_token_sequence(&tokens);
        assert_eq!(expected_error, result.err().unwrap());
    }

    #[test]
    fn calculate_single_numbers_test()
    {
        assert_eq!(1, calculate("1").unwrap());
        assert_eq!(123, calculate("123").unwrap());
        assert_eq!(123, calculate(" 123  ").unwrap());
    }

    #[test]
    fn build_tree_simple_test()
    {
        assert_eq!("265", run_build_tree_test("265"));
        assert_eq!("(1 + 2)", run_build_tree_test("1 + 2"));
        assert_eq!("(((1 + 2) - 684) + 84648)", run_build_tree_test("1 + 2 - 684 + 84648"));
    }

    #[test]
    fn build_tree_parantheses_test()
    {
        assert_eq!("((1 + 2) - (684 + 84648))", run_build_tree_test("1 - 2 - (684 + 84648)"));
    }

    fn run_build_tree_test(phrase: &str) -> String {
        return to_string_node(&build_tree(tokenize(phrase).unwrap()).unwrap());
    }

    #[test]
    fn calculate_simple_test()
    {
        assert_eq!(265, calculate("265").unwrap());
        assert_eq!(1 + 2, calculate("1 + 2").unwrap());
        assert_eq!(1 + 2 - 684 + 84648, calculate("1 + 2 - 684 + 84648").unwrap());
    }

    #[test]
    fn calculate_parantheses_test()
    {
        assert_eq!(1 + 2 - (684 + 84648), calculate("1 - 2 - (684 + 84648)").unwrap());
    }

    fn to_string_node(node: &EvaluationNode) -> String {
        match node {
            EvaluationNode::Number(num) => num.to_string(),
            EvaluationNode::Complex(left, op, right) =>
                {
                    let l = to_string_node(&left);
                    let o = to_string_op(&op);
                    let r = to_string_node(&right);
                    return "(".to_owned() + &l + " " + &o + " " + &r + ")";
                }
        }
    }

    fn to_string_op(op: &Operator) -> String
    {
        match op {
            Operator::Add => "+".to_string(),
            Operator::Subtract => "-".to_string(),
            Operator::Multiply => "*".to_string(),
            Operator::Divide => "/".to_string()
        }
    }
}
