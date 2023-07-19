use std::io::Write;
use std::{collections::HashMap, io::Error};

use itertools::FoldWhile::Continue;
use itertools::Itertools;

use crate::search_dictionary::search_dictionary;

enum UmlautVariant {
    A,
    O,
    U,
}

enum DigraphCounter {
    None,
    Umlaut(UmlautVariant),
    Eszett,
}

pub(crate) fn replace_digraphs(
    mut input: HashMap<String, Vec<Vec<u8>>>,
    dictionary: Vec<Vec<u8>>,
) -> Result<(), Error> {
    for (file_name, lines) in input.iter_mut() {
        for line in lines.iter_mut() {
            let words: Vec<&mut [u8]> = line.split_mut(|&x| x == b' ').collect();

            let mut new_line: Vec<u8> = Vec::new();

            for word in words.into_iter() {
                let mut digraph_indices: HashMap<usize, DigraphCounter> = HashMap::new();

                word.iter()
                    .enumerate()
                    .fold(DigraphCounter::None, |acc, (index, &x)| match x {
                        b'a' => match acc {
                            DigraphCounter::None => DigraphCounter::Umlaut(UmlautVariant::A),
                            _ => DigraphCounter::None,
                        },
                        b'o' => match acc {
                            DigraphCounter::None => DigraphCounter::Umlaut(UmlautVariant::O),
                            _ => DigraphCounter::None,
                        },
                        b'u' => match acc {
                            DigraphCounter::None => DigraphCounter::Umlaut(UmlautVariant::U),
                            _ => DigraphCounter::None,
                        },
                        b'e' => match acc {
                            DigraphCounter::Umlaut(umlaut_variant) => {
                                digraph_indices
                                    .insert(index - 1, DigraphCounter::Umlaut(umlaut_variant));
                                DigraphCounter::None
                            }
                            _ => DigraphCounter::None,
                        },
                        b's' => match acc {
                            DigraphCounter::None => DigraphCounter::Eszett,
                            DigraphCounter::Eszett => {
                                digraph_indices.insert(index - 1, DigraphCounter::Eszett);
                                DigraphCounter::None
                            }
                            _ => DigraphCounter::None,
                        },
                        _ => match acc {
                            _ => DigraphCounter::None,
                        },
                    });

                if digraph_indices.is_empty() {
                    new_line.extend(word.to_vec());
                    new_line.push(b' ');
                    continue;
                }

                // split replacement_word into non-alphabetical and alphabetical parts
                let mut replacement_word: Vec<u8> = Vec::new();

                let (prefix, suffix) = word
                    .to_vec()
                    .iter()
                    .fold_while(
                        (Vec::<u8>::new(), Vec::<u8>::new()),
                        |(mut prefix, mut suffix), &x| match x {
                            b'a'..=b'z' | b'A'..=b'Z' => {
                                replacement_word.push(x);
                                Continue((prefix, suffix))
                            }
                            _ => {
                                if replacement_word.is_empty() {
                                    prefix.push(x);
                                } else {
                                    suffix.push(x);
                                }
                                Continue((prefix, suffix))
                            }
                        },
                    )
                    .into_inner();

                for (digraph_index, digraph_counter) in digraph_indices.iter() {
                    match digraph_counter {
                        DigraphCounter::Umlaut(umlaut_variant) => match umlaut_variant {
                            UmlautVariant::A => {
                                replacement_word[*digraph_index] = b'\xE4';
                                replacement_word.remove(digraph_index + 1);
                            }
                            UmlautVariant::O => {
                                replacement_word[*digraph_index] = b'\xF6';
                                replacement_word.remove(digraph_index + 1);
                            }
                            UmlautVariant::U => {
                                replacement_word[*digraph_index] = b'\xFC';
                                replacement_word.remove(digraph_index + 1);
                            }
                        },
                        DigraphCounter::Eszett => {
                            replacement_word[*digraph_index] = b'\xDF';
                            replacement_word.remove(digraph_index + 1);
                        }
                        _ => continue,
                    }
                }

                match search_dictionary(replacement_word.clone(), &dictionary) {
                    true => {
                        new_line.extend(prefix);
                        new_line.extend(replacement_word.clone());
                        new_line.extend(suffix);
                        new_line.push(b' ');
                    }
                    false => {
                        new_line.extend(word.to_vec());
                        new_line.push(b' ');
                    }
                }
            }

            new_line.pop();

            *line = new_line;
        }

        let mut output_file = std::fs::File::create(format!("./output/{}", file_name))?;

        for line in lines {
            let line = line
                .iter()
                .filter(|&x| *x != b'\0')
                .map(|&x| x)
                .collect_vec();

            output_file.write_all(&line)?;
            output_file.write_all(b"\r\n")?;
        }
    }

    Ok(())
}
