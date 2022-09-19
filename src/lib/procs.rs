use crate::operators::{Condition, Proposition, Operator};

pub fn demorg(prop: Proposition) -> Proposition {
    match prop {
        Proposition::Predicate(pred) => Proposition::Predicate(pred), 
        Proposition::Composition(comp) => match *comp {
            Operator::And(a, b) => Proposition::Composition(Box::new(Operator::And(a, b))), 
            Operator::Or(a, b) => Proposition::Composition(Box::new(Operator::Or(a, b))), 
            Operator::Implies(a, b) => Proposition::Composition(Box::new(Operator::Implies(a, b))), 
            Operator::Not(a) => match a {
                Proposition::Predicate(pred) => Proposition::Composition(Box::new(Operator::Not(Proposition::Predicate(pred)))),
                Proposition::Condition(cond) => Proposition::Condition(match cond {
                    Condition::True => Condition::False,
                    Condition::False => Condition::True,
                }),
                Proposition::Composition(comp) => match *comp {
                    Operator::And(a, b) => Proposition::Composition(Box::new(Operator::Or(
                        demorg(Proposition::Composition(Box::new(Operator::Not(a)))), 
                        demorg(Proposition::Composition(Box::new(Operator::Not(b))))
                    ))),
                    Operator::Or(a, b) => Proposition::Composition(Box::new(Operator::And(
                        demorg(Proposition::Composition(Box::new(Operator::Not(a)))),
                        demorg(Proposition::Composition(Box::new(Operator::Not(b))))
                    ))),
                    Operator::Implies(a, b) => Proposition::Composition(Box::new(Operator::And(demorg(a), Proposition::Composition(Box::new(Operator::Not(b)))))),
                    Operator::Not(a) => demorg(a),
                }
            }
        },
        Proposition::Condition(cond) => Proposition::Condition(cond),
    }
}

#[cfg(test)]
mod test_procs {
    use super::*;

    #[test]
    fn test_demorg() {
        let input = vec![
            Proposition::Composition(Box::new(Operator::Not(Proposition::Composition(Box::new(Operator::And(
                                    Proposition::Predicate("A".into()),
                                    Proposition::Predicate("A".into())
            ))))))
        ];
        let expected = vec![
            Proposition::Composition(Box::new(Operator::Or(
                        Proposition::Composition(Box::new(Operator::Not(Proposition::Predicate("A".into())))),
                        Proposition::Composition(Box::new(Operator::Not(Proposition::Predicate("A".into()))))
            )))
        ];

        input.into_iter().zip(expected.into_iter()).for_each(|(actual, expected)| assert_eq!(expected, demorg(actual)));
    }
}
