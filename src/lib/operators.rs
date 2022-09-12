use crate::stream;

#[derive(Debug)]
pub enum Operator {
    And(Proposition, Proposition),
    Or(Proposition, Proposition),
    Implies(Proposition, Proposition),
    Not(Proposition),
}

impl TryInto<Proposition> for crate::stream::TokenStream {
    type Error = ParseError;

    fn try_into(self) -> Result<Proposition, Self::Error> {
        if self.0.len() < 1 {
            return Err(ParseError)
        }
        
        fn parse_prop(i: &mut usize, stream: &stream::TokenStream) -> Result<Proposition, ParseError> {
            let mut first = *i;
            match &stream.0[first] {
                stream::Token::Predicate(A) => {
                    *i += 1;
                    if first + 1 < stream.0.len() {
                        match &stream.0[first + 1] {
                            stream::Token::Bracket(stream::Bracket::Close) => {
                                *i += 1;
                                return Ok(Proposition::Predicate(A.clone()));
                            },
                            stream::Token::Operator(op) => {
                                *i += 1;
                                return match op {
                                    stream::Operator::And => {
                                        Ok(Proposition::Composition(Box::new(Operator::And(Proposition::Predicate(A.clone()), parse_prop(i, stream)?))))
                                    },
                                    stream::Operator::Or => {
                                        Ok(Proposition::Composition(Box::new(Operator::Or(Proposition::Predicate(A.clone()), parse_prop(i, stream)?))))
                                    },
                                    stream::Operator::Implies => {
                                        Ok(Proposition::Composition(Box::new(Operator::Implies(Proposition::Predicate(A.clone()), parse_prop(i, stream)?))))
                                    },
                                    stream::Operator::Not => {
                                        Err(ParseError)
                                    }
                                }
                            },
                            _ => Err(ParseError)
                        }
                    } else {
                        return Ok(Proposition::Predicate(A.clone()))
                    }
                },
                stream::Token::Bracket(stream::Bracket::Open) => {
                    *i += 1;
                    let prop = parse_prop(i, stream)?;
                    first = *i;

                    if first + 1 < stream.0.len() {
                        match &stream.0[first] {
                            stream::Token::Bracket(stream::Bracket::Close) => {
                                *i += 1;
                                return Ok(prop);
                            },
                            stream::Token::Operator(op) => {
                                *i += 1;
                                return match op {
                                    stream::Operator::And => {
                                        Ok(Proposition::Composition(Box::new(Operator::And(prop, parse_prop(i, stream)?))))
                                    },
                                    stream::Operator::Or => {
                                        Ok(Proposition::Composition(Box::new(Operator::Or(prop, parse_prop(i, stream)?))))
                                    },
                                    stream::Operator::Implies => {
                                        Ok(Proposition::Composition(Box::new(Operator::Implies(prop, parse_prop(i, stream)?))))
                                    },
                                    stream::Operator::Not => {
                                        Err(ParseError)
                                    }
                                }
                            },
                            _ => Err(ParseError)
                        }
                    } else {
                        return Ok(prop)
                    }
                },
                stream::Token::Operator(stream::Operator::Not) => {
                    *i += 1;
                    println!("found not!");
                    return Ok(Proposition::Composition(Box::new(Operator::Not(parse_prop(i, stream)?))));
                },
                _ => Err(ParseError)
            } 
        }
        let mut i = 0;
        
        parse_prop(&mut i, &self)
    }
}

pub struct ParseError;

#[derive(Debug)]
pub enum Proposition {
    Condition(Condition),
    Predicate(String),
    Composition(Box<Operator>),
}

#[derive(Debug)]
pub enum Condition {
    True,
    False,
}

#[cfg(test)]
mod test_operators {
    use super::*;

    #[test]
    fn test_composition() {
        let comp = Proposition::Composition(Box::new(Operator::And(Proposition::Predicate("A".to_string()), Proposition::Predicate("B".to_string()))));        
        println!("{:?}", comp);
    }

    #[test]
    fn test_complex_compostion() {
        let comp = Proposition::Composition(Box::new(Operator::And(
                    Proposition::Predicate("C".to_string()),
                    Proposition::Composition(Box::new(Operator::Or(
                                Proposition::Predicate("A".to_string()),
                                Proposition::Predicate("B".to_string())
                                )))
                    )));        
        println!("{:?}", comp)
    }


    #[test]
    fn test_parsing() {
        use stream::{Token, Bracket};
        let stream = stream::TokenStream(vec![
            Token::Predicate("A".to_string()), 
            Token::Operator(stream::Operator::And), 
            Token::Bracket(Bracket::Open),
            Token::Predicate("B".to_string()),
            Token::Operator(stream::Operator::Or),
            Token::Predicate("C".to_string()),
            Token::Bracket(Bracket::Close),
        ]);
        let comp: Proposition = stream.try_into().ok().unwrap();
        println!("{:?}", comp)
    }
}
