use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub(crate) fn search_dictionary(word: Vec<u8>, dictionary: Rc<RefCell<TreeNode>>) -> bool {
    let mut current_node = dictionary;

    for letter in word {
        let child = current_node.borrow().get_child(letter);

        match child {
            Some(child) => current_node = child,
            None => return false,
        }
    }

    true
}
