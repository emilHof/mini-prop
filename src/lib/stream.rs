#[derive(Debug)]
pub struct TokenStream(Vec<Token>);

#[derive(Debug)]
pub enum Token {
    Bracket(Bracket),
    Operator(Operator),
    Predicate(String),
}

#[derive(Debug)]
pub enum Operator {
    And,
    Or,
    Not,
}

#[derive(Debug)]
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
                        "land" => {
                            res.push(Token::Operator(Operator::And));
                        },
                        "lor" => {
                            res.push(Token::Operator(Operator::Or));
                        },
                        a => res.push(Token::Predicate(a.to_string()))
                    }
                }
                buf = vec![];
            } else if c == ')' {
                if buf.len() > 0 {
                    match buf.clone().into_iter().collect::<String>().as_str() {
                        "land" => {
                            res.push(Token::Operator(Operator::And));
                        },
                        "lor" => {
                            res.push(Token::Operator(Operator::Or));
                        },
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
}
