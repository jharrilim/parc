use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[end]
    End,

    #[error]
    Error,

    #[token = "{"]
    LeftCurlyBoi,

    #[token = "}"]
    RightCurlyBoi,

    #[token = "("]
    LeftParenBoi,

    #[token = ")"]
    RightParenBoi,

    #[regex = "[a-zA-Z]+"]
    Identifier,

    #[token = "function"]
    Function
}


pub fn main() {
    let mut lexer = Token::lexer("function fooBar (){}");
    assert_eq!(lexer.token, Token::Function);
    lexer.advance();
    assert_eq!(lexer.token, Token::Identifier);
    lexer.advance();
    assert_eq!(lexer.token, Token::LeftParenBoi);
    lexer.advance();
    assert_eq!(lexer.token, Token::RightParenBoi);
    lexer.advance();
    assert_eq!(lexer.token, Token::LeftCurlyBoi);
    lexer.advance();
    assert_eq!(lexer.token, Token::RightCurlyBoi);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
