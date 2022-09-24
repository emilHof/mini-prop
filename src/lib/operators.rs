use crate::stream;

#[derive(Debug, PartialEq)]
pub enum Operator {
    And(Proposition, Proposition),
    Or(Proposition, Proposition),
    Implies(Proposition, Proposition),
    Not(Proposition),
}

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug, PartialEq)]
pub enum Proposition {
    Condition(Condition),
    Predicate(String),
    Composition(Box<Operator>),
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    True,
    False,
}

impl TryInto<Proposition> for crate::stream::TokenStream {
    type Error = ParseError;

    fn try_into(self) -> Result<Proposition, Self::Error> {
        if self.0.len() < 1 {
            return Err(ParseError)
        }
        
        let mut i = 0;
        
        parse_prop(&mut i, &self)
    }
}


fn parse_prop(i: &mut usize, stream: &stream::TokenStream) -> Result<Proposition, ParseError> {
    let mut first = *i;
    match &stream.0[first] {
        stream::Token::Predicate(pred) => {
            *i += 1;
            if first + 1 < stream.0.len() {
                match &stream.0[first + 1] {
                    stream::Token::Bracket(stream::Bracket::Close) => {
                        *i += 1;
                        return Ok(Proposition::Predicate(pred.clone()));
                    },
                    stream::Token::Operator(op) => {
                        *i += 1;
                        return match_op(op, i, stream, Proposition::Predicate(pred.clone()));
                    },
                    _ => Err(ParseError)
                }
            } else {
                return Ok(Proposition::Predicate(pred.clone()))
            }
        },
        stream::Token::Bracket(stream::Bracket::Open) => {
            *i += 1;
            let prop = parse_prop(i, stream)?;
            first = *i;

            if first + 1 < stream.0.len() {
                match_op_prop(*i, i, stream, prop)
            } else {
                return Ok(prop)
            }
        },
        stream::Token::Operator(stream::Operator::Not) => {
            *i += 1;
            handle_not(i, stream)
        },
        _ => Err(ParseError)
    } 
}

pub fn handle_not(i: &mut usize, stream: &stream::TokenStream) -> Result<Proposition, ParseError> {
    let mut first = *i;
    if first < stream.0.len() {

        let prop = match &stream.0[first] {
            stream::Token::Predicate(pred) => { *i += 1; Ok(Proposition::Composition(Box::new(Operator::Not(Proposition::Predicate(pred.clone()))))) },
            stream::Token::Bracket(stream::Bracket::Open) => {
                *i += 1;
                Ok(Proposition::Composition(Box::new(Operator::Not(parse_prop(i, stream)?))))
            },
            _ => Err(ParseError)
        }?;
        first = *i;

        if first + 1 < stream.0.len() {
            match_op_prop(*i, i, stream, prop)
        } else {
            return Ok(prop)
        }
    } else {
        Err(ParseError)
    }
}

pub fn match_op_prop(first: usize, i: &mut usize, stream: &stream::TokenStream, prop: Proposition) -> Result<Proposition, ParseError> {
    match &stream.0[first] {
        stream::Token::Bracket(stream::Bracket::Close) => {
            *i += 1;
            return Ok(prop);
        },
        stream::Token::Operator(op) => {
            *i += 1;
            match_op(op, i, stream, prop)
        },
        _ => Err(ParseError)
    }
}

pub fn match_op(op: &stream::Operator, i: &mut usize, stream: &stream::TokenStream, prop: Proposition) -> Result<Proposition, ParseError> { 
    match op {
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
}

impl std::fmt::Display for Proposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Proposition::Predicate(pred) => write!(f, "{}", pred),
            Proposition::Condition(cond) => match cond {
                Condition::True => write!(f, "T"),
                Condition::False => write!(f, "F"),
            },
            Proposition::Composition(comp) => match comp.as_ref() {
                Operator::And(a, b) => write!(f, "({} \\land {})", a, b),
                Operator::Or(a, b) => write!(f, "({} \\lor {})", a, b),
                Operator::Implies(a, b) => write!(f, "({} \\implies {})", a, b),
               Operator::Not(a) => write!(f, "\\neg {}", a),            
            }
        }
    }
}

impl Into<String> for Proposition {
    fn into(self) -> String {
        format!("{}", self)
    }
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
        println!("{:?}", comp);
        println!("{}", comp);
    }


    #[test]
    fn test_parsing() {
        use stream::Token;
        let stream = stream::TokenStream(vec![Token::Predicate("A".to_string()), Token::Operator(stream::Operator::And), Token::Predicate("B".to_string())]);
        let comp: Proposition = stream.try_into().ok().unwrap();
        println!("{:?}", comp)
    }
}
