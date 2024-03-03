
fn main() {
    _ = calculate("1 + 3 * (3-1) / 2");
}

#[derive(Debug, PartialEq)]
enum Token
{
    Number(i32),
    Add,
    Subtract,
    Multiply,
    Divide,
    BeginParenthesis,
    EndParenthesis,
}
#[derive(Debug, PartialEq)]
enum CalculatorError{
    InvalidCharacter(char)
}
fn calculate(input: &str) -> Result<i32, CalculatorError>
{
    _ = tokenize(input);
    return Result::Ok(3);
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalculatorError> {
    let mut tokens = vec![];
    let mut current_number: Option<i32> = Option::None;
    for char in input.chars() {
       match char { 
           '1' => { current_number = append_to_number(1, &current_number); }
           '2' => { current_number = append_to_number(2, &current_number); }
           '3' => { current_number = append_to_number(3, &current_number); }
           '4' => { current_number = append_to_number(4, &current_number); }
           '5' => { current_number = append_to_number(5, &current_number); }
           '6' => { current_number = append_to_number(6, &current_number); }
           '7' => { current_number = append_to_number(7, &current_number); }
           '8' => { current_number = append_to_number(8, &current_number); }
           '9' => { current_number = append_to_number(9, &current_number); }
           '0' => { current_number = append_to_number(0, &current_number); }
           '+' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::Add); }
           '-' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::Subtract); }
           '*' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::Multiply); }
           '/' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::Divide); }
           '(' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::BeginParenthesis); }
           ')' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::EndParenthesis); }
           ' ' => {} // Skip any white spaces
           _ => return Result::Err(CalculatorError::InvalidCharacter(char))
       } 
    }
    if current_number.is_some() { 
        tokens.push(Token::Number(current_number.unwrap())); 
    }
   return Result::Ok(tokens); 
} 
fn append_to_number(digit: i32, current_number: &Option<i32>) -> Option<i32>
{
   let mut number = if current_number.is_some() { current_number.unwrap() }else{ 0 }; 
    number *= 10;
    number += digit;
    return Option::Some(number);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn tokenize1_test()
    {
       let tokens = tokenize("1 + 232*32-  21").unwrap();

        assert_eq!(7, tokens.len());
        assert_eq!(Token::Number(1), tokens[0]);
        assert_eq!(Token::Add, tokens[1]);
        assert_eq!(Token::Number(232), tokens[2]);
        assert_eq!(Token::Multiply, tokens[3]);
        assert_eq!(Token::Number(32), tokens[4]);
        assert_eq!(Token::Subtract, tokens[5]);
        assert_eq!(Token::Number(21), tokens[6]);
    }
    
    #[test]
    fn tokenize2_test()
    {
        let tokens = tokenize("(1 + 232*(32)-").unwrap();

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
    fn tokenize_invalid_char_expect_error_test()
    {
        let result = tokenize("1 +M 232*32-  21");
        assert_eq!( Result::Err(CalculatorError::InvalidCharacter('M')), result);
    }
}

