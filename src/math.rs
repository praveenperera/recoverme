pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

pub fn permuations(count_of_word_set: u128, number_of_words: u128) -> u128 {
    factorial(count_of_word_set) / factorial(count_of_word_set - number_of_words)
}
