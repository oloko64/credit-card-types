use std::{collections::HashMap, ops::DerefMut};

pub(crate) struct CardTypes(HashMap<&'static str, CardTypeInner>);

impl CardTypes {
    pub fn new() -> CardTypes {
        let cards = [
            (
                "visa",
                CardTypeInner {
                    nice_type: "Visa",
                    type_: "visa",
                    patterns: &[&[4]],
                    gaps: &[4, 8, 12],
                    lengths: &[16, 18, 19],
                    code: Code {
                        name: "CVV",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "mastercard",
                CardTypeInner {
                    nice_type: "Mastercard",
                    type_: "mastercard",
                    patterns: &[
                        &[51, 55],
                        &[2221, 2229],
                        &[223, 229],
                        &[23, 26],
                        &[270, 271],
                        &[2720],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[16],
                    code: Code {
                        name: "CVC",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "american-express",
                CardTypeInner {
                    nice_type: "American Express",
                    type_: "american-express",
                    patterns: &[&[34], &[37]],
                    gaps: &[4, 10],
                    lengths: &[15],
                    code: Code {
                        name: "CID",
                        size: 4,
                    },
                    match_strength: None,
                },
            ),
            (
                "diners-club",
                CardTypeInner {
                    nice_type: "Diners Club",
                    type_: "diners-club",
                    patterns: &[&[300, 305], &[36], &[38], &[39]],
                    gaps: &[4, 10],
                    lengths: &[14, 16, 19],
                    code: Code {
                        name: "CVV",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "discover",
                CardTypeInner {
                    nice_type: "Discover",
                    type_: "discover",
                    patterns: &[&[6011], &[644, 649], &[65]],
                    gaps: &[4, 8, 12],
                    lengths: &[16, 19],
                    code: Code {
                        name: "CID",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "jcb",
                CardTypeInner {
                    nice_type: "JCB",
                    type_: "jcb",
                    patterns: &[&[2131], &[1800], &[3528, 3589]],
                    gaps: &[4, 8, 12],
                    lengths: &[16, 17, 18, 19],
                    code: Code {
                        name: "CVV",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "unionpay",
                CardTypeInner {
                    nice_type: "UnionPay",
                    type_: "unionpay",
                    patterns: &[
                        &[620],
                        &[62100, 62182],
                        &[62184, 62187],
                        &[62185, 62197],
                        &[62200, 62205],
                        &[622010, 622999],
                        &[622018],
                        &[62207, 62209],
                        &[623, 626],
                        &[6270],
                        &[6272],
                        &[6276],
                        &[627700, 627779],
                        &[627781, 627799],
                        &[6282, 6289],
                        &[6291],
                        &[6292],
                        &[810],
                        &[8110, 8131],
                        &[8132, 8151],
                        &[8152, 8163],
                        &[8164, 8171],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[14, 15, 16, 17, 18, 19],
                    code: Code {
                        name: "CVN",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "maestro",
                CardTypeInner {
                    nice_type: "Maestro",
                    type_: "maestro",
                    patterns: &[
                        &[493698],
                        &[500000, 504174],
                        &[504176, 506698],
                        &[506779, 508999],
                        &[56, 59],
                        &[63],
                        &[67],
                        &[6],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[12, 13, 14, 15, 16, 17, 18, 19],
                    code: Code {
                        name: "CVC",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "elo",
                CardTypeInner {
                    nice_type: "Elo",
                    type_: "elo",
                    patterns: &[
                        &[401178],
                        &[401179],
                        &[438935],
                        &[457631],
                        &[457632],
                        &[431274],
                        &[451416],
                        &[457393],
                        &[504175],
                        &[506699, 506778],
                        &[509000, 509999],
                        &[627780],
                        &[636297],
                        &[636368],
                        &[650031, 650033],
                        &[650035, 650051],
                        &[650405, 650439],
                        &[650485, 650538],
                        &[650541, 650598],
                        &[650700, 650718],
                        &[650720, 650727],
                        &[650901, 650978],
                        &[651652, 651679],
                        &[655000, 655019],
                        &[655021, 655058],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[16],
                    code: Code {
                        name: "CVE",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "mir",
                CardTypeInner {
                    nice_type: "Mir",
                    type_: "mir",
                    patterns: &[&[2200, 2204]],
                    gaps: &[4, 8, 12],
                    lengths: &[16, 17, 18, 19],
                    code: Code {
                        name: "CVP2",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "hyper",
                CardTypeInner {
                    nice_type: "Hiper",
                    type_: "hiper",
                    patterns: &[
                        &[637095],
                        &[63737423],
                        &[63743358],
                        &[637568],
                        &[637599],
                        &[637609],
                        &[637612],
                    ],
                    gaps: &[4, 8, 12],
                    lengths: &[16],
                    code: Code {
                        name: "CVC",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
            (
                "hypercard",
                CardTypeInner {
                    nice_type: "Hipercard",
                    type_: "hipercard",
                    patterns: &[&[606282]],
                    gaps: &[4, 8, 12],
                    lengths: &[16],
                    code: Code {
                        name: "CVC",
                        size: 3,
                    },
                    match_strength: None,
                },
            ),
        ];
        let card_types = HashMap::from(cards);

        CardTypes(card_types)
    }

    pub fn insert_card_type(&mut self, card_type: CardTypeInner) {
        self.0.insert(card_type.type_, card_type);
    }

    pub fn get_credit_card_type(
        &mut self,
        card_number: impl AsRef<str>,
    ) -> Option<Vec<CardTypeInner>> {
        let card_number = card_number.as_ref();

        if card_number.is_empty() {
            return None;
        }

        let mut results = Vec::new();

        let mut all_cards = self.get_all_card_types();

        all_cards.iter_mut().for_each(|card_type| {
            add_best_match_to_results(card_number, card_type, &mut results);
        });

        let best_match = find_best_match(&mut results);

        if let Some(best_match) = best_match {
            return Some(vec![best_match]);
        }

        Some(
            results
                .iter()
                .map(|card_type| (*card_type).clone())
                .collect(),
        )
    }

    pub fn get_all_card_types(&mut self) -> Vec<&mut CardTypeInner> {
        self.0.values_mut().collect()
    }
}

fn find_best_match<'a>(results: &'a mut [&mut CardTypeInner]) -> Option<CardTypeInner> {
    if !can_determine_best_match(results) {
        return None;
    }

    let mut best_match_result = None;

    results.iter_mut().for_each(|card_type| {
        if best_match_result.is_none() {
            best_match_result = Some(card_type);
        } else if card_type.match_strength > best_match_result.as_ref().unwrap().match_strength {
            best_match_result = Some(card_type);
        }
    });

    best_match_result.map(|card_type| (*card_type).clone())
}

fn can_determine_best_match(results: &[&mut CardTypeInner]) -> bool {
    let number_of_results_with_max_strength = results
        .iter()
        .filter(|card_type| card_type.match_strength > Some(1))
        .count();

    number_of_results_with_max_strength > 1 && number_of_results_with_max_strength == results.len()
}

fn add_best_match_to_results<'a, 'b>(
    card_number: &str,
    card_type: &'b mut CardTypeInner,
    results: &'a mut Vec<&'b mut CardTypeInner>,
) {
    use crate::utils::matches;

    for pattern in card_type.patterns.iter() {
        if !matches(card_number, pattern) {
            continue;
        }

        let pattern_length = pattern.len();

        if card_number.len() >= pattern_length {
            card_type.match_strength = Some(pattern_length as i32);
        }

        results.push(card_type);
        break;
    }
}

#[derive(Debug, Clone)]
pub struct CardTypeInner {
    pub nice_type: &'static str,
    pub type_: &'static str,
    pub patterns: &'static [&'static [i32]],
    pub gaps: &'static [i32],
    pub lengths: &'static [i32],
    pub code: Code,
    pub match_strength: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub name: &'static str,
    pub size: i32,
}
