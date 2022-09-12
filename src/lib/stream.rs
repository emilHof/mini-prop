#[derive(Debug, PartialEq, Eq)]
pub struct TokenStream(pub Vec<Token>);

#[derive(Debug, PartialEq, Eq
    )]
pub enum Token {
    Bracket(Bracket),
    Operator(Operator),
    Predicate(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    And,
    Or,
    Not,
    Implies,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Bracket {
    Open,
    Close,
}

impl TryInto<TokenStream> for String {
    type Error = String;

    fn try_into(self) -> Result<TokenStream, Self::Error> {
        let mut buf: Vec<char> = vec![];
        let mut res = vec![];
        self.chars().for_each(|c| {
            if c == ' ' || c == '\\' {
                if buf.len() > 0 {
                    match buf.clone().into_iter().collect::<String>().as_str() {
                        "land" => res.push(Token::Operator(Operator::And)),
                        "lor" => res.push(Token::Operator(Operator::Or)),
                        "not" => res.push(Token::Operator(Operator::Not)),
                        "implies" => res.push(Token::Operator(Operator::Implies)),
                        a => res.push(Token::Predicate(a.to_string()))
                    }
                }
                buf = vec![];
            } else if c == ')' {
                if buf.len() > 0 {
                    match buf.clone().into_iter().collect::<String>().as_str() {
                        "land" => res.push(Token::Operator(Operator::And)),
                        "lor" => res.push(Token::Operator(Operator::Or)),
                        "not" => res.push(Token::Operator(Operator::Not)),
                        "implies" => res.push(Token::Operator(Operator::Implies)),
                        a => res.push(Token::Predicate(a.to_string()))
                    }
                }
                res.push(Token::Bracket(Bracket::Close));
                buf = vec![];
            } else if c == '(' {
                res.push(Token::Bracket(Bracket::Open));
                buf = vec![];
            } else {
                buf.push(c);
            }
        });

        Ok(TokenStream(res))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stream_from() {
        let input = "(A \\land B)".to_string();

        let sut: TokenStream = input.try_into().ok().unwrap(); 

        println!("{:?}", sut);
    }

    #[test]
    fn test_stream_from_complex() {
        let input = "A \\lor ((B \\land C) \\lor (D \\land \\not A))".to_string();

        let sut: TokenStream = input.try_into().ok().unwrap(); 

        println!("{:?}", sut);
    }
}
