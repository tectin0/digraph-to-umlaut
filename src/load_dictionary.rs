use crate::config::DICTIONARY_RELATIVE_PATH;

pub(crate) fn load_dictionary() -> Vec<Vec<u8>> {
    // append dictionary_relative_path to root path

    let root_path = std::env::current_dir().expect("Could not get current directory");

    let dictionary_path = root_path.join(DICTIONARY_RELATIVE_PATH);

    const HEADER_SIZE: usize = 17;

    let mut dictionary: Vec<Vec<u8>> = Vec::new();

    let file = std::fs::read(dictionary_path).unwrap();

    let is_crlf = file.iter().any(|&x| x == b'\r');

    let lines: Vec<Vec<u8>> = file
        .split(|&x| x == b'\n')
        .skip(HEADER_SIZE)
        .map(|x| {
            let mut line = x.to_vec();
            if is_crlf {
                line.pop();
            }
            line
        })
        .collect();

    // cannot convert to utf8 because of umlauts
    for line in lines {
        // remove `/` and everything after it
        let mut word = line;
        if let Some(index) = word.iter().position(|&x| x == b'/') {
            word.truncate(index);
        }
        dictionary.push(word);
    }

    println!("Dictionary size: {}", dictionary.len());

    return dictionary;
}
