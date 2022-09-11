#[derive(Debug)]
pub enum Operator {
    And(Proposition, Proposition),
    Or(Proposition, Proposition),
    Implies(Proposition, Proposition),
}

#[derive(Debug)]
pub enum Proposition {
    Predicate,
    Compostion(Box<Operator>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_composition() {
        let comp = Proposition::Compostion(Box::new(Operator::And(Proposition::Predicate, Proposition::Predicate)));        
        println!("{:?}", comp);
    }
}
