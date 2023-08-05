use crate::{errors::CardTypeError, CreditCardType};

pub fn matches(card_number: &str, pattern: &'static [&'static str]) -> Result<bool, CardTypeError> {
    if pattern.len() == 1 {
        return Ok(matches_pattern(card_number, pattern[0]));
    }

    match_range(card_number, pattern[0], pattern[1])
}

fn matches_pattern(card_number: &str, pattern: &'static str) -> bool {
    pattern.get(..card_number.len()).unwrap_or(pattern)
        == card_number.get(..pattern.len()).unwrap_or(card_number)
}

fn match_range(
    card_number: &str,
    min: &'static str,
    max: &'static str,
) -> Result<bool, CardTypeError> {
    let mut max_len_to_check = max.len();
    if max_len_to_check > card_number.len() {
        max_len_to_check = card_number.len();
    }
    let str_slice = &card_number[..max_len_to_check];
    let int_representation = str_slice.parse::<i32>()?;

    let min = &min[..str_slice.len()];
    let max = &max[..str_slice.len()];

    let min_int_representation = min.parse::<i32>()?;
    let max_int_representation = max.parse::<i32>()?;

    Ok(
        int_representation >= min_int_representation
            && int_representation <= max_int_representation,
    )
}

pub fn find_best_match<'a>(results: &'a mut [&mut CreditCardType]) -> Option<&'a CreditCardType> {
    if !can_determine_best_match(results) {
        return None;
    }

    let mut best_match_result: Option<&&mut CreditCardType> = None;

    for card_type in results.iter() {
        if best_match_result.is_none()
            || card_type.match_strength > best_match_result.as_ref()?.match_strength
        {
            best_match_result = Some(card_type);
        }
    }

    best_match_result.map(std::ops::Deref::deref)
}

fn can_determine_best_match(results: &[&mut CreditCardType]) -> bool {
    let number_of_results_with_max_strength = results
        .iter()
        .filter(|card_type| card_type.match_strength >= 1)
        .count();

    number_of_results_with_max_strength > 1 && number_of_results_with_max_strength == results.len()
}

pub fn add_best_match_to_results<'a>(
    card_number: &str,
    card_type: &'a mut CreditCardType,
    results: &mut Vec<&'a mut CreditCardType>,
) -> Result<(), CardTypeError> {
    for pattern in card_type.patterns.iter() {
        if !matches(card_number, pattern)? {
            continue;
        }

        let pattern_length = pattern[0].len();

        if card_number.len() >= pattern_length {
            card_type.match_strength = u32::try_from(pattern_length)?;
        }

        results.push(card_type);
        break;
    }

    Ok(())
}
