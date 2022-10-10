use super::*;

impl Proposition {
    pub fn demorg(self) -> Proposition {
        match self {
            Proposition::Predicate(pred) => Proposition::Predicate(pred), 
            Proposition::Composition(comp) => match *comp {
                Operator::And(a, b) => Proposition::new_and(a.demorg(), b.demorg()), 
                Operator::Or(a, b) => Proposition::new_or(a.demorg(), b.demorg()), 
                Operator::Implies(a, b) => Proposition::new_or(a, Proposition::new_not(b)).demorg(),
                    Operator::Not(a) => match a {
                    Proposition::Predicate(pred) => Proposition::new_not(pred),
                        Proposition::Condition(cond) => Proposition::Condition(match cond {
                        Condition::True => Condition::False,
                        Condition::False => Condition::True,
                    }),
                    Proposition::Composition(comp) => match *comp {
                        Operator::And(a, b) => Proposition::new_or(
                            Proposition::new_not(a).demorg(),
                            Proposition::new_not(b).demorg()
                        ),
                        Operator::Or(a, b) => Proposition::new_and(
                            Proposition::new_not(a).demorg(),
                            Proposition::new_not(b).demorg()
                        ),
                        Operator::Implies(a, b) => Proposition::new_and(Proposition::new_not(a), b).demorg(),
                        Operator::Not(a) => a.demorg(),
                    }
                }
            },
            Proposition::Condition(cond) => Proposition::Condition(cond),
        }
    }

    pub fn normal(self) -> Proposition {
        match self.demorg() {
            Proposition::Predicate(pred) => pred.into(),
            Proposition::Condition(cond) => Proposition::Condition(cond),
            Proposition::Composition(comp) => match *comp {
                Operator::And(a, b) => {
                    match a.normal() {
                        Proposition::Predicate(a) => {
                            let a = Proposition::Predicate(a);
                            match b.normal() {
                                Proposition::Predicate(b) => Proposition::new_and(a, b),
                                Proposition::Condition(cond) => Proposition::new_and(a, Proposition::Condition(cond)),
                                Proposition::Composition(comp) => match *comp {
                                    Operator::And(b, c) => Proposition::new_and(a, Proposition::new_and(b, c).normal()),
                                    Operator::Or(b, c) => Proposition::new_or(Proposition::new_and(a.clone(), b).normal(), Proposition::new_and(a, c).normal()),
                                    Operator::Not(b) => Proposition::new_and(a, Proposition::new_not(b)),
                                    Operator::Implies(_, _) => unreachable!(),
                                }
                            }
                        },
                        Proposition::Condition(a) => {
                            let a = Proposition::Condition(a);
                            match b.normal() {
                                Proposition::Predicate(b) => Proposition::new_and(a, Proposition::Predicate(b)),
                                Proposition::Condition(cond) => Proposition::new_and(a, Proposition::Condition(cond)),
                                Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, c) => Proposition::new_and(a, Proposition::new_and(b, c)),
                                    Operator::Or(b, c) => Proposition::new_or(Proposition::new_and(a.clone(), b), Proposition::new_and(a, c)),
                                    Operator::Not(b) => Proposition::new_and(a, Proposition::new_not(b)),
                                    Operator::Implies(_, _) => unreachable!(),
                                },
                            }
                        },
                        Proposition::Composition(a) => match *a {
                            Operator::And(a, c) => {
                                let a = Proposition::new_and(a, c);
                                Proposition::Composition(match b.normal() {
                                    Proposition::Predicate(b) => Box::new(Operator::And(a, Proposition::Predicate(b))),
                                    Proposition::Condition(cond) => Box::new(Operator::And(a, Proposition::Condition(cond))),
                                    Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, c) => Box::new(Operator::And(a, Proposition::Composition(Box::new(Operator::And(b, c))))),
                                        Operator::Or(b, c) => Box::new(Operator::Or(Proposition::Composition(Box::new(Operator::And(a.clone(), b))), Proposition::Composition(Box::new(Operator::And(a, c))))),
                                        Operator::Not(b) => Box::new(Operator::And(a, Proposition::Composition(Box::new(Operator::Not(b))))),
                                        Operator::Implies(_, _) => unreachable!(),
                                    }
                                })
                            },
                            Operator::Or(a, c) => {
                                Proposition::Composition(match b.normal() {
                                    Proposition::Predicate(b) => Box::new(Operator::Or(
                                        Proposition::new_and(a, b.clone()), 
                                        Proposition::Composition(Box::new(Operator::And(c, Proposition::Predicate(b.clone()))))
                                    )),
                                    Proposition::Condition(cond) => {
                                        let b = Proposition::Condition(cond);
                                        Box::new(Operator::Or(Proposition::Composition(Box::new(Operator::And(a, b.clone()))), Proposition::Composition(Box::new(Operator::And(c, b)))))
                                    },
                                    Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, d) => {
                                            let b = Proposition::new_and(b, d).normal();
                                            let a = Proposition::new_and(a, b.clone()).normal();
                                            let c = Proposition::new_and(c, b.clone()).normal();

                                            Box::new(Operator::Or(a, c)) 
                                        },
                                        Operator::Or(b, d) => {
                                            let ab = Proposition::new_and(a.clone(), b.clone()).normal();
                                            let ad = Proposition::new_and(a.clone(), d.clone()).normal();
                                            let cb = Proposition::new_and(c.clone(), b.clone()).normal();
                                            let cd = Proposition::new_and(c.clone(), d.clone()).normal();

                                            Box::new(Operator::Or(ab, Proposition::Composition(Box::new(Operator::Or(ad, Proposition::Composition(Box::new(Operator::Or(cb, cd))))))))
                                        },
                                        Operator::Not(b) => {
                                            let b = Proposition::Composition(Box::new(Operator::Not(b)));

                                            Box::new(Operator::Or(Proposition::Composition(Box::new(Operator::And(a, b.clone()))), Proposition::Composition(Box::new(Operator::And(c, b)))))
                                        },
                                        Operator::Implies(_, _) => unreachable!(),
                                    }
                                }).normal()
                            },
                            Operator::Not(a) => {
                                let a = Proposition::Composition(Box::new(Operator::Not(a)));
                                Proposition::Composition(match b.normal() {
                                    Proposition::Predicate(b) => Box::new(Operator::And(a, Proposition::Predicate(b))),
                                    Proposition::Condition(cond) => Box::new(Operator::And(a, Proposition::Condition(cond))),
                                    Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, c) => Box::new(Operator::And(a, Proposition::Composition(Box::new(Operator::And(b, c))))),
                                        Operator::Or(b, c) => Box::new(Operator::Or(
                                            Proposition::Composition(Box::new(Operator::And(a.clone(), b))), 
                                            Proposition::Composition(Box::new(Operator::And(a, c)))
                                        )),
                                        Operator::Not(b) => Box::new(Operator::And(a, Proposition::Composition(Box::new(Operator::Not(b))))),
                                        Operator::Implies(_, _) => unreachable!(),
                                    }
                                })
                            },
                            Operator::Implies(_, _) => unreachable!(),
                        },
                    }
                },
                Operator::Or(a, b) => Proposition::new_or(a.normal(), b.normal()),
                Operator::Implies(a, b) => Proposition::new_or(a.normal(),Proposition::new_not(b.normal())),
                Operator::Not(a) => Proposition::new_not(a.normal()),
            }
        }
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

        input.into_iter().zip(expected.into_iter()).for_each( |(actual, expected)| {
                let actual = actual.demorg();
                println!("{:?}, {:?}", expected, actual);
                assert_eq!(expected, actual)
            }
        );
    }

    #[test]
    fn test_normal() {
        let cases = vec![
            (
                Proposition::new_and("A", Proposition::new_or("B", Proposition::new_not("C"))),
                Proposition::new_or(Proposition::new_and("A", "B"), Proposition::new_and("A", Proposition::new_not("C")))
            ),
            (
                Proposition::new_and("A", Proposition::new_or("B", Proposition::new_not(Proposition::new_and("C", "D")))),
                Proposition::new_or(Proposition::new_and("A", "B"), Proposition::new_or(
                    Proposition::new_and(
                        "A", 
                        Proposition::new_not("C")
                    ), 
                    Proposition::new_and("A", Proposition::new_not("D"))
                )),
            ),
            (
                Proposition::new_and(Proposition::new_or("A", "C"), Proposition::new_or("B", "D")),
                Proposition::new_or(Proposition::new_and("A", "B"), Proposition::new_or(
                    Proposition::new_and("A", "D"), 
                    Proposition::new_or(Proposition::new_and("C", "B"), Proposition::new_and("C", "D"))
                ))
            ),
            (
                // ((A \lor B) \land C) \land (D \lor E)
                // (( A \land C ) \lor (B \land C)) \land (D \lor E)
                // (A \land C \land D) \lor (A \land C \land E) \lor (B \land C \land D) \lor (B \land C \land E)
                Proposition::new_and(Proposition::new_and(Proposition::new_or("A", "B"), "C"), Proposition::new_or("D", "E")),
                Proposition::new_or(
                    Proposition::new_and(Proposition::new_and("A", "C"), "D"), 
                    Proposition::new_or(
                        Proposition::new_and(Proposition::new_and("A", "C"), "E"), 
                        Proposition::new_or(
                            Proposition::new_and(Proposition::new_and("B", "C"), "D"), 
                            Proposition::new_and(Proposition::new_and("B", "C"), "E")
                        )
                    )
                )
            ),
            (
                // \neg ((A \lor B) \implies ((C \land D) \lor E))
                // (\neg A \land \neg B) \land ((C \land D) \lor E)
                // (\neg A \land \neg B \land C \land D) \lor (\neg A \land \neg B \land E)
                Proposition::new_not(Proposition::new_implies(Proposition::new_or("A", "B"), Proposition::new_or(Proposition::new_and("C", "D"), "E"))),
                Proposition::new_or(
                    Proposition::new_and(Proposition::new_and(Proposition::new_not("A"), Proposition::new_not("B")), Proposition::new_and("C", "D")), 
                    Proposition::new_and(Proposition::new_and(Proposition::new_not("A"), Proposition::new_not("B")), "E") 
                )
            )
        ];

        cases.into_iter().for_each(|(input, expected)| {
            let actual = input.normal();
            println!("{}", &actual);
            assert_eq!(expected, actual)
        })
    }
}
