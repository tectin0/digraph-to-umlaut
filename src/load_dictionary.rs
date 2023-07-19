use std::{cell::RefCell, error::Error, rc::Rc};

use crate::{config::get_config_value, tree::TreeNode, utils::ToUmlautLowercase};

pub(crate) fn load_dictionary() -> Result<Rc<RefCell<TreeNode>>, Box<dyn Error>> {
    let ignore_case = crate::config::get_config_value("ignore_case")?
        .as_bool()
        .unwrap();

    let root_path = std::env::current_dir()?;

    let data_path = root_path.join(get_config_value("data_path")?.as_str().unwrap());

    let files = std::fs::read_dir(data_path)?;

    let mut lines: Vec<Vec<u8>> = Vec::new();

    for file in files.into_iter() {
        let file = std::fs::read(file?.path()).unwrap();

        let is_crlf = file.iter().any(|&x| x == b'\r');

        let mut lines_iter = file.split(|&x| x == b'\n');

        while let Some(line) = lines_iter.next() {
            if line.is_empty() {
                continue;
            }

            if line[0] == b'#' || line[0] == b' ' {
                continue;
            }

            let mut line = line.to_vec();

            if is_crlf {
                line.pop();
            }

            lines.push(line);
        }
    }

    let dictionary = Rc::new(RefCell::new(TreeNode::new(None)));

    while let Some(line) = lines.pop() {
        let word_affix = line;

        let (mut word, _affix) = {
            let mut split = word_affix.split(|&x| x == b'/');
            let word = split.next().unwrap();
            let affix = match split.next() {
                Some(affix) => affix,
                None => &[],
            };
            (word.to_vec(), affix.to_vec())
        };

        if ignore_case {
            word.make_ascii_lowercase();
            word.make_umlaut_lowercase();
        }

        let mut current = dictionary.clone();

        for letter in word.iter() {
            let child = current.borrow_mut().get_child(*letter);
            match child {
                Some(child) => {
                    current = child;
                }
                None => {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(Some(*letter))));
                    current.borrow_mut().add_child(new_node.clone());
                    current = new_node;
                }
            }
        }
    }

    return Ok(dictionary);
}
