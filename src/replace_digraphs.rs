use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::rc::Rc;

use itertools::izip;
use itertools::FoldWhile::Continue;
use itertools::Itertools;

use crate::constants::*;

use crate::config;
use crate::search_dictionary::search_dictionary;
use crate::tree::TreeNode;
use crate::utils::ToUmlautLowercase;

enum Case {
    Lower,
    Upper,
}

enum UmlautVariant {
    A(Case),
    O(Case),
    U(Case),
}

enum DigraphCounter {
    None,
    Umlaut(UmlautVariant),
    Eszett,
}

pub(crate) fn replace_digraphs(
    mut input: HashMap<String, Vec<Vec<u8>>>,
    dictionary: Rc<RefCell<TreeNode>>,
) -> Result<(), Box<dyn Error>> {
    let ignore_case = config::get_config_value("ignore_case")?.as_bool().unwrap();

    for (file_name, lines) in input.iter_mut() {
        for line in lines.iter_mut() {
            let mut word_delimiters: Vec<u8> = Vec::new();

            let words: Vec<&mut [u8]> = line
                .split_mut(|&x| match x {
                    b' ' => {
                        word_delimiters.push(x);
                        true
                    }
                    b'-' => {
                        word_delimiters.push(x);
                        true
                    }
                    _ => false,
                })
                .collect();

            word_delimiters.push(b' ');

            let mut new_line: Vec<u8> = Vec::new();

            assert_eq!(words.len(), word_delimiters.len());

            for (word, delimiter) in izip!(words.into_iter(), word_delimiters.into_iter()) {
                let mut replacement_word: Vec<u8> = Vec::new();

                // split replacement_word into non-alphabetical and alphabetical parts
                // TODO: extract
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

                let mut digraphs: HashMap<usize, DigraphCounter> = HashMap::new();

                let mut max_digraph_index = 0;

                replacement_word.iter().enumerate().fold(
                    DigraphCounter::None,
                    |acc, (index, &x)| match x {
                        b'a' => match acc {
                            _ => DigraphCounter::Umlaut(UmlautVariant::A(Case::Lower)),
                        },
                        b'A' => match acc {
                            _ => DigraphCounter::Umlaut(UmlautVariant::A(Case::Upper)),
                        },
                        b'o' => match acc {
                            _ => DigraphCounter::Umlaut(UmlautVariant::O(Case::Lower)),
                        },
                        b'O' => match acc {
                            _ => DigraphCounter::Umlaut(UmlautVariant::O(Case::Upper)),
                        },
                        b'u' => match acc {
                            _ => DigraphCounter::Umlaut(UmlautVariant::U(Case::Lower)),
                        },
                        b'U' => match acc {
                            _ => DigraphCounter::Umlaut(UmlautVariant::U(Case::Upper)),
                        },
                        b'e' => match acc {
                            DigraphCounter::Umlaut(umlaut_variant) => {
                                let digraph_index = index - 1;
                                max_digraph_index = max_digraph_index.max(digraph_index);

                                digraphs
                                    .insert(digraph_index, DigraphCounter::Umlaut(umlaut_variant));

                                DigraphCounter::None
                            }
                            _ => DigraphCounter::None,
                        },
                        b's' => match acc {
                            DigraphCounter::Eszett => {
                                let digraph_index = index - 1;
                                max_digraph_index = max_digraph_index.max(digraph_index);

                                digraphs.insert(digraph_index, DigraphCounter::Eszett);

                                DigraphCounter::None
                            }
                            _ => DigraphCounter::Eszett,
                        },
                        _ => match acc {
                            _ => DigraphCounter::None,
                        },
                    },
                );

                if digraphs.is_empty() {
                    new_line.extend(word.to_vec());
                    new_line.push(delimiter);
                    continue;
                }

                for (digraph_index, digraph_counter) in digraphs.iter() {
                    match digraph_counter {
                        DigraphCounter::Umlaut(umlaut_variant) => match umlaut_variant {
                            UmlautVariant::A(case) => {
                                replacement_word[*digraph_index] = match case {
                                    Case::Lower => LOWER_AE,
                                    Case::Upper => UPPER_AE,
                                };
                            }
                            UmlautVariant::O(case) => {
                                replacement_word[*digraph_index] = match case {
                                    Case::Lower => LOWER_OE,
                                    Case::Upper => UPPER_OE,
                                };
                            }
                            UmlautVariant::U(case) => {
                                replacement_word[*digraph_index] = match case {
                                    Case::Lower => LOWER_UE,
                                    Case::Upper => UPPER_UE,
                                };
                            }
                        },
                        DigraphCounter::Eszett => {
                            replacement_word[*digraph_index] = ESZETT;
                        }
                        _ => continue,
                    }
                }

                let mut digraph_indices = digraphs.keys().collect_vec();
                digraph_indices.sort();

                for digraph_index in digraph_indices.into_iter().rev() {
                    replacement_word.remove(*digraph_index + 1);
                }

                let search_cut_off = max_digraph_index + 1 - digraphs.len();

                match search_dictionary(
                    {
                        match ignore_case {
                            true => {
                                let mut search_word = replacement_word.clone();
                                search_word.make_ascii_lowercase();
                                search_word.make_umlaut_lowercase();
                                search_word[..=search_cut_off].to_vec()
                            }
                            false => replacement_word.clone()[..=search_cut_off].to_vec(),
                        }
                    },
                    dictionary.clone(),
                ) {
                    true => {
                        new_line.extend(prefix);
                        new_line.extend(replacement_word.clone());
                        new_line.extend(suffix);
                        new_line.push(delimiter);
                    }
                    false => {
                        new_line.extend(word.to_vec());
                        new_line.push(delimiter);
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
