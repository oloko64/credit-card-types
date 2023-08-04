pub(crate) fn matches(card_number: &str, pattern: &'static [i32]) -> bool {
    if pattern.len() == 1 {
        return matches_pattern(card_number, pattern[0]);
    }

    match_range(card_number, pattern[0], pattern[1])
}

fn matches_pattern(card_number: &str, pattern: i32) -> bool {
    let pattern = pattern.to_string();

    pattern.get(..card_number.len()).unwrap_or(&pattern)
        == card_number.get(..pattern.len()).unwrap_or(&card_number)
}

fn match_range(card_number: &str, min: i32, max: i32) -> bool {
    let mut max_len_to_check = max.to_string().len();
    if max_len_to_check > card_number.len() {
        max_len_to_check = card_number.len();
    }
    let str_slice = &card_number[..max_len_to_check];
    let int_repr = str_slice.parse::<i32>().unwrap();

    let min = &min.to_string()[..str_slice.len()];
    let max = &max.to_string()[..str_slice.len()];

    int_repr >= min.parse::<i32>().unwrap() && int_repr <= max.parse::<i32>().unwrap()
}
