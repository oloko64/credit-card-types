use std::collections::BTreeMap;

pub(crate) struct CardTypes(BTreeMap<&'static str, CardTypeInner>);

impl CardTypes {
    pub fn new() -> CardTypes {
        let cards = [
            (
                "visa",
                CardTypeInner {
                    nice_type: "Visa",
                    type_: "visa",
                    patterns: &[&["4"]],
                    gaps: &[4, 8, 12],
                    lengths: &[16, 18, 19],
                    code: Code {
                        name: "CVV",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "mastercard",
                CardTypeInner {
                    nice_type: "Mastercard",
                    type_: "mastercard",
                    patterns: &[
                        &["51", "55"],
                        &["2221", "2229"],
                        &["223", "229"],
                        &["23", "26"],
                        &["270", "271"],
                        &["2720"],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[16],
                    code: Code {
                        name: "CVC",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "american-express",
                CardTypeInner {
                    nice_type: "American Express",
                    type_: "american-express",
                    patterns: &[&["34"], &["37"]],
                    gaps: &[4, 10],
                    lengths: &[15],
                    code: Code {
                        name: "CID",
                        size: 4,
                    },
                    match_strength: 0,
                },
            ),
            (
                "diners-club",
                CardTypeInner {
                    nice_type: "Diners Club",
                    type_: "diners-club",
                    patterns: &[&["300", "305"], &["36"], &["38"], &["39"]],
                    gaps: &[4, 10],
                    lengths: &[14, 16, 19],
                    code: Code {
                        name: "CVV",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "discover",
                CardTypeInner {
                    nice_type: "Discover",
                    type_: "discover",
                    patterns: &[&["6011"], &["644", "649"], &["65"]],
                    gaps: &[4, 8, 12],
                    lengths: &[16, 19],
                    code: Code {
                        name: "CID",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "jcb",
                CardTypeInner {
                    nice_type: "JCB",
                    type_: "jcb",
                    patterns: &[&["2131"], &["1800"], &["3528", "3589"]],
                    gaps: &[4, 8, 12],
                    lengths: &[16, 17, 18, 19],
                    code: Code {
                        name: "CVV",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "unionpay",
                CardTypeInner {
                    nice_type: "UnionPay",
                    type_: "unionpay",
                    patterns: &[
                        &["620"],
                        &["62100", "62182"],
                        &["62184", "62187"],
                        &["62185", "62197"],
                        &["62200", "62205"],
                        &["622010", "622999"],
                        &["622018"],
                        &["62207", "62209"],
                        &["623", "626"],
                        &["6270"],
                        &["6272"],
                        &["6276"],
                        &["627700", "627779"],
                        &["627781", "627799"],
                        &["6282", "6289"],
                        &["6291"],
                        &["6292"],
                        &["810"],
                        &["8110", "8131"],
                        &["8132", "8151"],
                        &["8152", "8163"],
                        &["8164", "8171"],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[14, 15, 16, 17, 18, 19],
                    code: Code {
                        name: "CVN",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "maestro",
                CardTypeInner {
                    nice_type: "Maestro",
                    type_: "maestro",
                    patterns: &[
                        &["493698"],
                        &["500000", "504174"],
                        &["504176", "506698"],
                        &["506779", "508999"],
                        &["56", "59"],
                        &["63"],
                        &["67"],
                        &["6"],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[12, 13, 14, 15, 16, 17, 18, 19],
                    code: Code {
                        name: "CVC",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "elo",
                CardTypeInner {
                    nice_type: "Elo",
                    type_: "elo",
                    patterns: &[
                        &["401178"],
                        &["401179"],
                        &["438935"],
                        &["457631"],
                        &["457632"],
                        &["431274"],
                        &["451416"],
                        &["457393"],
                        &["504175"],
                        &["506699", "506778"],
                        &["509000", "509999"],
                        &["627780"],
                        &["636297"],
                        &["636368"],
                        &["650031", "650033"],
                        &["650035", "650051"],
                        &["650405", "650439"],
                        &["650485", "650538"],
                        &["650541", "650598"],
                        &["650700", "650718"],
                        &["650720", "650727"],
                        &["650901", "650978"],
                        &["651652", "651679"],
                        &["655000", "655019"],
                        &["655021", "655058"],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[16],
                    code: Code {
                        name: "CVE",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "mir",
                CardTypeInner {
                    nice_type: "Mir",
                    type_: "mir",
                    patterns: &[&["2200", "2204"]],
                    gaps: &[4, 8, 12],
                    lengths: &[16, 17, 18, 19],
                    code: Code {
                        name: "CVP2",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "hyper",
                CardTypeInner {
                    nice_type: "Hiper",
                    type_: "hiper",
                    patterns: &[
                        &["637095"],
                        &["63737423"],
                        &["63743358"],
                        &["637568"],
                        &["637599"],
                        &["637609"],
                        &["637612"],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[16],
                    code: Code {
                        name: "CVC",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
            (
                "hypercard",
                CardTypeInner {
                    nice_type: "Hipercard",
                    type_: "hipercard",
                    patterns: &[&["606282"]],
                    gaps: &[4, 8, 12],
                    lengths: &[16],
                    code: Code {
                        name: "CVC",
                        size: 3,
                    },
                    match_strength: 0,
                },
            ),
        ];
        let card_types = BTreeMap::from(cards);

        CardTypes(card_types)
    }

    pub fn insert_card_type(&mut self, card_type: CardTypeInner) {
        self.0.insert(card_type.type_, card_type);
    }

    pub fn get_credit_card_type(
        &self,
        card_number: impl AsRef<str>,
    ) -> Result<Vec<CardTypeInner>, std::num::ParseIntError> {
        let card_number = card_number.as_ref();

        if card_number.is_empty() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        let mut all_cards = self.get_all_card_types();

        for card_type in all_cards.iter_mut() {
            add_best_match_to_results(card_number, card_type, &mut results)?
        }

        let best_match = find_best_match(&mut results);

        if let Some(best_match) = best_match {
            return Ok(vec![best_match]);
        }

        Ok(results
            .iter()
            .map(|card_type| (*card_type).clone())
            .collect())
    }

    pub fn get_all_card_types(&self) -> Vec<CardTypeInner> {
        self.0
            .values()
            .map(|card_type| (*card_type).clone())
            .collect()
    }
}

fn find_best_match<'a>(results: &'a mut [&mut CardTypeInner]) -> Option<CardTypeInner> {
    if !can_determine_best_match(results) {
        return None;
    }

    let mut best_match_result = None;

    for card_type in results.iter_mut() {
        if best_match_result.is_none() {
            best_match_result = Some(card_type);
        } else if card_type.match_strength > best_match_result.as_ref()?.match_strength {
            best_match_result = Some(card_type);
        }
    }

    best_match_result.map(|card_type| (*card_type).clone())
}

fn can_determine_best_match(results: &[&mut CardTypeInner]) -> bool {
    let number_of_results_with_max_strength = results
        .iter()
        .filter(|card_type| card_type.match_strength >= 1)
        .count();

    number_of_results_with_max_strength > 1 && number_of_results_with_max_strength == results.len()
}

fn add_best_match_to_results<'a, 'b>(
    card_number: &str,
    card_type: &'b mut CardTypeInner,
    results: &'a mut Vec<&'b mut CardTypeInner>,
) -> Result<(), std::num::ParseIntError> {
    use crate::utils::matches;

    for pattern in card_type.patterns.iter() {
        if !matches(card_number, pattern)? {
            continue;
        }

        let pattern_length = pattern[0].len();

        if card_number.len() >= pattern_length {
            card_type.match_strength = pattern_length as u32;
        }

        results.push(card_type);
        break;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CardTypeInner {
    pub nice_type: &'static str,
    pub type_: &'static str,
    pub patterns: &'static [&'static [&'static str]],
    pub gaps: &'static [i32],
    pub lengths: &'static [i32],
    pub code: Code,
    pub match_strength: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    pub name: &'static str,
    pub size: i32,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    enum Matcher {
        CardNumber(&'static str),
        ShouldMatch(&'static [&'static str]),
        ShouldMatchCode(Code),
    }

    impl Matcher {
        fn get_card_number(&self) -> &'static str {
            match self {
                Matcher::CardNumber(card_number) => card_number,
                _ => panic!("Should not be called"),
            }
        }

        fn get_should_match(&self) -> &'static [&'static str] {
            match self {
                Matcher::ShouldMatch(should_match) => should_match,
                _ => panic!("Should not be called"),
            }
        }

        fn get_code(&self) -> Code {
            match self {
                Matcher::ShouldMatchCode(code) => code.clone(),
                _ => panic!("Should not be called"),
            }
        }
    }

    #[test]
    fn test_single_card_types() {
        let card_tests = [
            ["411", "visa"],
            ["4111111111111111", "visa"],
            ["4012888888881881", "visa"],
            ["4222222222222", "visa"],
            ["4462030000000000", "visa"],
            ["4484070000000000", "visa"],
            ["411111111111111111", "visa"],
            ["4111111111111111110", "visa"],
            ["431274", "elo"],
            ["451416", "elo"],
            ["457393", "elo"],
            ["401178", "elo"],
            ["401179", "elo"],
            ["438935", "elo"],
            ["457631", "elo"],
            ["457632", "elo"],
            ["4576321111111111", "elo"],
            ["5066991111111118", "elo"],
            ["504175", "elo"],
            ["6277809", "elo"],
            ["6277809990229178", "elo"],
            ["650033", "elo"],
            ["6500331111111111", "elo"],
            ["2221", "mastercard"],
            ["2222", "mastercard"],
            ["2223", "mastercard"],
            ["2224", "mastercard"],
            ["2225", "mastercard"],
            ["2226", "mastercard"],
            ["2225", "mastercard"],
            ["2226", "mastercard"],
            ["223", "mastercard"],
            ["2239", "mastercard"],
            ["23", "mastercard"],
            ["24", "mastercard"],
            ["25", "mastercard"],
            ["26", "mastercard"],
            ["27", "mastercard"],
            ["270", "mastercard"],
            ["271", "mastercard"],
            ["272", "mastercard"],
            ["2720", "mastercard"],
            ["51", "mastercard"],
            ["52", "mastercard"],
            ["53", "mastercard"],
            ["54", "mastercard"],
            ["55", "mastercard"],
            ["5555555555554444", "mastercard"],
            ["5454545454545454", "mastercard"],
            ["34", "american-express"],
            ["37", "american-express"],
            ["341", "american-express"],
            ["34343434343434", "american-express"],
            ["378282246310005", "american-express"],
            ["371449635398431", "american-express"],
            ["378734493671000", "american-express"],
            ["30", "diners-club"],
            ["300", "diners-club"],
            ["36", "diners-club"],
            ["38", "diners-club"],
            ["39", "diners-club"],
            ["30569309025904", "diners-club"],
            ["38520000023237", "diners-club"],
            ["36700102000000", "diners-club"],
            ["36148900647913", "diners-club"],
            ["6011", "discover"],
            ["644", "discover"],
            ["644", "discover"],
            ["645", "discover"],
            ["646", "discover"],
            ["647", "discover"],
            ["648", "discover"],
            ["649", "discover"],
            ["6011000400000000", "discover"],
            ["6011111111111117", "discover"],
            ["6011000990139424", "discover"],
            ["62123456789002", "unionpay"],
            ["621234567890003", "unionpay"],
            ["6221258812340000", "unionpay"],
            ["622018111111111111", "unionpay"],
            ["6212345678900000003", "unionpay"],
            ["56", "maestro"],
            ["57", "maestro"],
            ["58", "maestro"],
            ["59", "maestro"],
            ["67", "maestro"],
            ["6304000000000000", "maestro"],
            ["6799990100000000019", "maestro"],
            ["62183", "maestro"],
            ["1", "jcb"],
            ["35", "jcb"],
            ["2131", "jcb"],
            ["21312", "jcb"],
            ["1800", "jcb"],
            ["18002", "jcb"],
            ["3530111333300000", "jcb"],
            ["3566002020360505", "jcb"],
            ["35308796121637357", "jcb"],
            ["353445444300732639", "jcb"],
            ["3537286818376838569", "jcb"],
            ["6221260000000000", "unionpay"],
            ["6221260000000000000", "unionpay"],
            ["6222000000000000", "unionpay"],
            ["6228000000000000", "unionpay"],
            ["6229250000000000", "unionpay"],
            ["6229250000000000000", "unionpay"],
            ["6240000000000000", "unionpay"],
            ["6260000000000000000", "unionpay"],
            ["6282000000000000", "unionpay"],
            ["6289000000000000000", "unionpay"],
            ["6221558812340000", "unionpay"],
            ["6269992058134322", "unionpay"],
            ["622018111111111111", "unionpay"],
            ["8", "unionpay"],
            ["8100513433325374", "unionpay"],
            ["8111700872004845", "unionpay"],
            ["8141618644273338", "unionpay"],
            ["8158163233706018", "unionpay"],
            ["8168524506870054", "unionpay"],
            ["220", "mir"],
            ["2200", "mir"],
            ["2204", "mir"],
            ["22000000000000000", "mir"],
            ["22049999999999999", "mir"],
            ["6062820524845321", "hipercard"],
            ["6062820000", "hipercard"],
            ["6370950000000005", "hiper"],
            ["637095", "hiper"],
            ["637609", "hiper"],
            ["637599", "hiper"],
            ["637612", "hiper"],
            ["637568", "hiper"],
            ["63737423", "hiper"],
            ["63743358", "hiper"],
        ];

        let card_types = CardTypes::new();

        for test in card_tests.iter() {
            let card_type = card_types.get_credit_card_type(test[0]).unwrap();
            assert_eq!(card_type[0].type_, test[1], "Failed for {}", test[0]);
        }
    }

    #[test]
    fn test_multiple_card_types() {
        use Matcher::*;
        let card_tests = [
            ([CardNumber(""), ShouldMatch(&[])]),
            ([CardNumber("2"), ShouldMatch(&["mastercard", "jcb", "mir"])]),
            ([
                CardNumber("3"),
                ShouldMatch(&["american-express", "diners-club", "jcb"]),
            ]),
            ([
                CardNumber("5"),
                ShouldMatch(&["mastercard", "maestro", "elo"]),
            ]),
            ([CardNumber("50"), ShouldMatch(&["maestro", "elo"])]),
            [
                CardNumber("6"),
                ShouldMatch(&[
                    "discover",
                    "unionpay",
                    "maestro",
                    "elo",
                    "hiper",
                    "hipercard",
                ]),
            ],
            [
                CardNumber("60"),
                ShouldMatch(&["discover", "maestro", "hipercard"]),
            ],
            [CardNumber("601"), ShouldMatch(&["discover", "maestro"])],
            [CardNumber("64"), ShouldMatch(&["discover", "maestro"])],
            [
                CardNumber("62"),
                ShouldMatch(&["unionpay", "maestro", "elo"]),
            ],
            [CardNumber("4"), ShouldMatch(&["visa", "maestro", "elo"])],
            [CardNumber("43"), ShouldMatch(&["visa", "elo"])],
            [CardNumber("431"), ShouldMatch(&["visa", "elo"])],
            [CardNumber("4312"), ShouldMatch(&["visa", "elo"])],
            [CardNumber("43127"), ShouldMatch(&["visa", "elo"])],
            [CardNumber("45141"), ShouldMatch(&["visa", "elo"])],
            [CardNumber("45739"), ShouldMatch(&["visa", "elo"])],
            [CardNumber("40117"), ShouldMatch(&["visa", "elo"])],
            [CardNumber("43893"), ShouldMatch(&["visa", "elo"])],
            [CardNumber("45763"), ShouldMatch(&["visa", "elo"])],
            [
                CardNumber("6277"),
                ShouldMatch(&["unionpay", "maestro", "elo"]),
            ],
            [
                CardNumber("62778"),
                ShouldMatch(&["unionpay", "maestro", "elo"]),
            ],
            [CardNumber("63"), ShouldMatch(&["maestro", "elo", "hiper"])],
            [CardNumber("636"), ShouldMatch(&["maestro", "elo"])],
            [CardNumber("6362"), ShouldMatch(&["maestro", "elo"])],
            [CardNumber("63629"), ShouldMatch(&["maestro", "elo"])],
            [CardNumber("637"), ShouldMatch(&["maestro", "hiper"])],
            [CardNumber("637374"), ShouldMatch(&["maestro", "hiper"])],
            [CardNumber("637433"), ShouldMatch(&["maestro", "hiper"])],
            [CardNumber("606"), ShouldMatch(&["maestro", "hipercard"])],
            [
                CardNumber("627"),
                ShouldMatch(&["unionpay", "maestro", "elo"]),
            ],
            [CardNumber("6062"), ShouldMatch(&["maestro", "hipercard"])],
            [CardNumber("6370"), ShouldMatch(&["maestro", "hiper"])],
            [CardNumber("6376"), ShouldMatch(&["maestro", "hiper"])],
            [CardNumber("6375"), ShouldMatch(&["maestro", "hiper"])],
            [
                CardNumber("65"),
                ShouldMatch(&["discover", "maestro", "elo"]),
            ],
            [
                CardNumber("655"),
                ShouldMatch(&["discover", "maestro", "elo"]),
            ],
            [
                CardNumber("6550"),
                ShouldMatch(&["discover", "maestro", "elo"]),
            ],
            [
                CardNumber("65502"),
                ShouldMatch(&["discover", "maestro", "elo"]),
            ],
        ];

        let card_types = CardTypes::new();

        for test in card_tests.iter() {
            let card_types = card_types
                .get_credit_card_type(test[0].get_card_number())
                .unwrap();
            let mut card_names = card_types.iter().map(|card| card.type_).collect::<Vec<_>>();
            card_names.sort();
            let mut should_match = test[1].get_should_match().to_vec();
            should_match.sort();
            assert_eq!(
                card_names,
                should_match,
                "Failed for {}",
                test[0].get_card_number()
            );
        }
    }

    #[test]
    fn test_invalid_card_types() {
        let card_tests = [
            "0", "12", "123", "181", "1802", "221", "222099", "2721", "212", "2132", "306", "31",
            "32", "33", "7", "9",
        ];

        let card_types = CardTypes::new();
        for test in card_tests.iter() {
            let card_type = card_types.get_credit_card_type(test).unwrap();
            assert_eq!(card_type.len(), 0, "Failed for {}", test);
        }
    }

    #[test]
    fn test_card_ccv() {
        use Matcher::*;

        let card_tests = [
            [
                CardNumber("5454545454545454"),
                ShouldMatchCode(Code {
                    size: 3,
                    name: "CVC",
                }),
            ],
            [
                CardNumber("4111111111111111"),
                ShouldMatchCode(Code {
                    size: 3,
                    name: "CVV",
                }),
            ],
            [
                CardNumber("378734493671000"),
                ShouldMatchCode(Code {
                    size: 4,
                    name: "CID",
                }),
            ],
            [
                CardNumber("6011000990139424"),
                ShouldMatchCode(Code {
                    size: 3,
                    name: "CID",
                }),
            ],
            [
                CardNumber("30569309025904"),
                ShouldMatchCode(Code {
                    size: 3,
                    name: "CVV",
                }),
            ],
            [
                CardNumber("30569309025904"),
                ShouldMatchCode(Code {
                    size: 3,
                    name: "CVV",
                }),
            ],
            [
                CardNumber("6220558812340000"),
                ShouldMatchCode(Code {
                    size: 3,
                    name: "CVN",
                }),
            ],
            [
                CardNumber("6304000000000000"),
                ShouldMatchCode(Code {
                    size: 3,
                    name: "CVC",
                }),
            ],
            [
                CardNumber("2200000000000000"),
                ShouldMatchCode(Code {
                    size: 3,
                    name: "CVP2",
                }),
            ],
        ];

        let card_types = CardTypes::new();

        for test in card_tests.iter() {
            let card_types = card_types
                .get_credit_card_type(test[0].get_card_number())
                .unwrap();
            let card_names = card_types
                .iter()
                .map(|card| card.code.clone())
                .collect::<Vec<_>>();
            let should_match = test[1].get_code();
            assert_eq!(
                card_names,
                vec![should_match],
                "Failed for {}",
                test[0].get_card_number()
            );
        }
    }
}
