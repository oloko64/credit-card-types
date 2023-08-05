pub(crate) fn matches(
    card_number: &str,
    pattern: &'static [&'static str],
) -> Result<bool, std::num::ParseIntError> {
    if pattern.len() == 1 {
        return Ok(matches_pattern(card_number, pattern[0]));
    }

    Ok(match_range(card_number, pattern[0], pattern[1])?)
}

fn matches_pattern(card_number: &str, pattern: &'static str) -> bool {
    pattern.get(..card_number.len()).unwrap_or(&pattern)
        == card_number.get(..pattern.len()).unwrap_or(&card_number)
}

fn match_range(
    card_number: &str,
    min: &'static str,
    max: &'static str,
) -> Result<bool, std::num::ParseIntError> {
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
