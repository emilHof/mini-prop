use super::*;

impl Proposition {
    pub fn demorg(self) -> Proposition {
        match self {
            Proposition::Predicate(pred) => Proposition::Predicate(pred), 
            Proposition::Composition(comp) => match *comp {
                Operator::And(a, b) => Proposition::Composition(Box::new(Operator::And(a.demorg(), b.demorg()))), 
                Operator::Or(a, b) => Proposition::Composition(Box::new(Operator::Or(a.demorg(), b.demorg()))), 
                Operator::Implies(a, b) => Proposition::Composition(Box::new(Operator::Implies(a.demorg(), b.demorg()))), 
                Operator::Not(a) => match a {
                    Proposition::Predicate(pred) => Proposition::Composition(Box::new(Operator::Not(Proposition::Predicate(pred)))),
                    Proposition::Condition(cond) => Proposition::Condition(match cond {
                        Condition::True => Condition::False,
                        Condition::False => Condition::True,
                    }),
                    Proposition::Composition(comp) => match *comp {
                        Operator::And(a, b) => Proposition::Composition(Box::new(Operator::Or(
                            Proposition::Composition(Box::new(Operator::Not(a))).demorg(),
                            Proposition::Composition(Box::new(Operator::Not(b))).demorg()
                        ))),
                        Operator::Or(a, b) => Proposition::Composition(Box::new(Operator::And(
                            Proposition::Composition(Box::new(Operator::Not(a))).demorg(),
                            Proposition::Composition(Box::new(Operator::Not(b))).demorg()
                        ))),
                        Operator::Implies(a, b) => Proposition::Composition(Box::new(Operator::And(a.demorg(), Proposition::Composition(Box::new(Operator::Not(b)))))),
                        Operator::Not(a) => a.demorg(),
                    }
                }
            },
            Proposition::Condition(cond) => Proposition::Condition(cond),
        }
    }

    pub fn normal(self) -> Proposition {
        match self.demorg() {
            Proposition::Predicate(pred) => Proposition::Predicate(pred),
            Proposition::Condition(cond) => Proposition::Condition(match cond {
                Condition::True => Condition::False,
                Condition::False => Condition::True,
            }),
            Proposition::Composition(comp) => match *comp {
                Operator::And(a, b) => {
                    match a.normal() {
                        Proposition::Predicate(a) => {
                            let a = Proposition::Predicate(a);
                            Proposition::Composition(match b.normal() {
                                Proposition::Predicate(b) => Box::new(Operator::And(a, Proposition::Predicate(b))),
                                Proposition::Condition(cond) => Box::new(Operator::And(a, Proposition::Condition(cond))),
                                Proposition::Composition(comp) => match *comp {
                                    Operator::And(b, c) => Box::new(Operator::And(a, Proposition::Composition(Box::new(Operator::And(b, c))).normal())),
                                    Operator::Or(b, c) => Box::new(Operator::Or(Proposition::Composition(Box::new(Operator::And(a.clone(), b))), Proposition::Composition(Box::new(Operator::And(a, c))).normal())),
                                    Operator::Not(b) => Box::new(Operator::And(a, Proposition::Composition(Box::new(Operator::Not(b))))),
                                    Operator::Implies(_, _) => unreachable!(),
                                }
                            })
                        },
                        Proposition::Condition(a) => {
                            let a = Proposition::Condition(a);
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
                        Proposition::Composition(a) => match *a {
                            Operator::And(a, c) => {
                                let a = Proposition::Composition(Box::new(Operator::And(a, c)));
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
                                        Proposition::Composition(Box::new(Operator::And(a, Proposition::Predicate(b.clone())))), 
                                        Proposition::Composition(Box::new(Operator::And(c, Proposition::Predicate(b.clone()))))
                                    )),
                                    Proposition::Condition(cond) => {
                                        let b = Proposition::Condition(cond);
                                        Box::new(Operator::Or(Proposition::Composition(Box::new(Operator::And(a, b.clone()))), Proposition::Composition(Box::new(Operator::And(c, b)))))
                                    },
                                    Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, d) => {
                                            let b = Proposition::Composition(Box::new(Operator::And(b, d))).normal();
                                            let a = Proposition::Composition(Box::new(Operator::And(a, b.clone()))).normal();
                                            let c = Proposition::Composition(Box::new(Operator::And(c, b.clone()))).normal();

                                            Box::new(Operator::Or(a, c)) 
                                        },
                                        Operator::Or(b, d) => {
                                            let ab = Proposition::Composition(Box::new(Operator::And(a.clone(), b.clone()))).normal();
                                            let ad = Proposition::Composition(Box::new(Operator::And(a.clone(), d.clone()))).normal();
                                            let cb = Proposition::Composition(Box::new(Operator::And(c.clone(), b.clone()))).normal();
                                            let cd = Proposition::Composition(Box::new(Operator::And(c.clone(), d.clone()))).normal();

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
                                        Operator::Or(b, c) => Box::new(Operator::Or(Proposition::Composition(Box::new(Operator::And(a.clone(), b))), Proposition::Composition(Box::new(Operator::And(a, c))))),
                                        Operator::Not(b) => Box::new(Operator::And(a, Proposition::Composition(Box::new(Operator::Not(b))))),
                                        Operator::Implies(_, _) => unreachable!(),
                                    }
                                })
                            },
                            Operator::Implies(_, _) => unreachable!(),
                        },
                    }
                },
                Operator::Or(a, b) => Proposition::Composition(Box::new(Operator::Or(a.normal(), b.normal()))),
                Operator::Implies(a, b) => Proposition::Composition(Box::new(Operator::Or(a.normal(), Proposition::Composition(Box::new(Operator::Not(b.normal()))).demorg()))),
                Operator::Not(a) => Proposition::Composition(Box::new(Operator::Not(a.normal()))),
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
                Proposition::new_or(Proposition::new_and("A", "B"), Proposition::new_or(Proposition::new_and("A", Proposition::new_not("C")), Proposition::new_and("A", Proposition::new_not("D")))),
            )
        ];

        cases.into_iter().for_each(|(input, expected)| {
            let actual = input.normal();
            println!("{}", &actual);
            assert_eq!(actual, expected)
        })
    }
}
