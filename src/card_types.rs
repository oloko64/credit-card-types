use std::collections::BTreeMap;

pub struct CreditCardPool(BTreeMap<&'static str, CreditCardType>);

impl CreditCardPool {
    pub fn new() -> CreditCardPool {
        let cards = [
            (
                "visa",
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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
                CreditCardType {
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

        CreditCardPool(card_types)
    }

    pub fn insert_card_type(&mut self, card_type: CreditCardType) {
        self.0.insert(card_type.type_, card_type);
    }

    pub fn get_credit_card_type(
        &self,
        card_number: impl AsRef<str>,
    ) -> Result<Vec<CreditCardType>, std::num::ParseIntError> {
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

    pub fn get_all_card_types(&self) -> Vec<CreditCardType> {
        self.0
            .values()
            .map(|card_type| (*card_type).clone())
            .collect()
    }
}

fn find_best_match<'a>(results: &'a mut [&mut CreditCardType]) -> Option<CreditCardType> {
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

fn can_determine_best_match(results: &[&mut CreditCardType]) -> bool {
    let number_of_results_with_max_strength = results
        .iter()
        .filter(|card_type| card_type.match_strength >= 1)
        .count();

    number_of_results_with_max_strength > 1 && number_of_results_with_max_strength == results.len()
}

fn add_best_match_to_results<'a, 'b>(
    card_number: &str,
    card_type: &'b mut CreditCardType,
    results: &'a mut Vec<&'b mut CreditCardType>,
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
pub struct CreditCardType {
    pub nice_type: &'static str,
    pub type_: &'static str,
    pub patterns: &'static [&'static [&'static str]],
    pub gaps: &'static [i32],
    pub lengths: &'static [i32],
    pub code: Code,
    pub match_strength: u32,
}

impl Default for CreditCardType {
    fn default() -> Self {
        Self {
            nice_type: "Default",
            type_: "default",
            patterns: &[&["123456789"]],
            gaps: &[4],
            lengths: &[16],
            code: Code {
                name: "CVV",
                size: 3,
            },
            match_strength: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Code {
    pub name: &'static str,
    pub size: i32,
}
