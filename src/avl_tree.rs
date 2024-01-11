#![allow(dead_code)]
use std::{ptr::NonNull, marker::PhantomData};
use std::cmp::{max, Ordering};

type Link<T> = Option<NonNull<AvlTreeNode<T>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct AvlTreeNode<T: Ord> {
    pub data: T,
    pub left: Link<T>,
    pub right: Link<T>,
    pub parent: Link<T>,
    pub height: usize,
}

impl<T: Ord> AvlTreeNode<T> {
    pub fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |left| unsafe { (*left.as_ptr()).height });
        let right_height = self.right.as_ref().map_or(0, |right| unsafe { (*right.as_ptr()).height });

        self.height = 1 + max(left_height, right_height);
    }

    pub fn balance_factor(&mut self) -> usize {
        let left_height = self.left
            .as_ref()
            .map_or(0, |left| unsafe { (*left.as_ptr()).height });
        let right_height = self.right
            .as_ref()
            .map_or(0, |right| unsafe { (*right.as_ptr()).height });

        right_height - left_height
    }
}

#[derive(Debug, Clone)]
pub struct AvlTree<T: Ord> {
    root: Link<T>,
    _marker: PhantomData<T>,
}

impl<T: Ord> AvlTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            _marker: PhantomData,
        }
    }

    pub fn insert(&mut self, data: T) -> bool {
        unsafe {
            let mut current_node = &mut self.root;
            while let Some(node_ptr) = current_node {
                let node = &mut (*node_ptr.as_ptr());
                match node.data.cmp(&data){
                    Ordering::Greater => current_node = &mut node.left,
                    Ordering::Equal => return false,
                    Ordering::Less => current_node = &mut node.right,
                }
            }
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(AvlTreeNode {
                data,
                left: None,
                right: None,
                parent: None,
                height: 0
            })));
            //TODO: Set parent as needed.
            *current_node = Some(new);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_insert() {
        let mut tree: AvlTree<i32> = AvlTree::new();
        assert!(tree.insert(1));
        assert!(!tree.insert(1));
        assert!(tree.insert(2));
        // TODO: Test if tree contains elements as expected
    }
}