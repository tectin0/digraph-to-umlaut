pub(crate) fn search_dictionary(word: Vec<u8>, dictionary: &Vec<Vec<u8>>) -> bool {
    dictionary.contains(&word)
}
