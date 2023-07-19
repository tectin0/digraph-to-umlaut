use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
pub(crate) struct TreeNode {
    pub(crate) value: Option<u8>,
    pub(crate) children: Vec<Rc<RefCell<TreeNode>>>,
    pub(crate) parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub(crate) fn new(value: Option<u8>) -> Self {
        Self {
            value,
            children: Vec::new(),
            parent: None,
        }
    }

    pub(crate) fn add_child(&mut self, child: Rc<RefCell<TreeNode>>) {
        self.children.push(child);
    }

    pub(crate) fn get_child(&self, value: u8) -> Option<Rc<RefCell<TreeNode>>> {
        for child in &self.children {
            if child.borrow().value == Some(value) {
                return Some(child.clone());
            }
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        self.children.len()
    }
}
