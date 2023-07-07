use std::error::Error;
use std::{cell::RefCell, rc::Rc};

use itertools::chain;

use crate::config::get_config_value;
use crate::tree::TreeNode;

use crate::constants::*;

use crate::utils::ToUmlautLowercase;

pub(crate) fn search_dictionary(
    word: Vec<u8>,
    dictionary: Rc<RefCell<TreeNode>>,
) -> Result<(bool, Option<Vec<u8>>), Box<dyn Error>> {
    let is_ignore_case = match get_config_value("ignore_case")?.as_bool() {
        Some(value) => value,
        None => false,
    };

    let mut current_node = dictionary;

    let mut eszett_node: Option<Rc<RefCell<TreeNode>>> = None;
    let mut is_eszett = false;

    for mut letter in word.clone().into_iter() {
        match is_ignore_case {
            true => {
                letter.make_ascii_lowercase();
                letter.make_umlaut_lowercase();
            }
            false => (),
        }

        let child = current_node.borrow().get_child(letter);

        if letter == ESZETT {
            is_eszett = true;
            eszett_node = Some(current_node.clone());
        }

        match child {
            Some(child) => current_node = child,
            None => match is_eszett {
                false => return Ok((false, None)),
                true => return Ok(recheck_eszett_tree_for_ss(eszett_node, &word)),
            },
        }
    }

    match is_eszett {
        true => match recheck_eszett_tree_for_ss(eszett_node, &word) {
            (true, Some(replacement_word)) => Ok((true, Some(replacement_word))),
            (true, None) => Ok((false, None)),
            (false, _) => Ok((true, None)),
        },
        false => Ok((true, None)),
    }
}

fn recheck_eszett_tree_for_ss(
    eszett_node: Option<Rc<RefCell<TreeNode>>>,
    word: &[u8],
) -> (bool, Option<Vec<u8>>) {
    let mut current_node = match eszett_node {
        Some(node) => node,
        None => return (false, None),
    };

    let mut split_word = word.split(|&x| x == ESZETT);

    let mut replacement_word = split_word.next().unwrap().to_vec();

    let letters = [b's', b's'];

    for letter in chain!(
        letters.into_iter(),
        split_word.next().unwrap().to_vec().into_iter()
    ) {
        let child: Option<Rc<RefCell<TreeNode>>> = current_node.borrow().get_child(letter);

        match child {
            Some(child) => {
                current_node = child;
                replacement_word.push(letter);
            }
            None => return (false, None),
        }
    }

    (true, Some(replacement_word))
}
