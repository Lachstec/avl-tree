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

pub struct Iter<'a, T: Ord> {
    prev_nodes: Vec<&'a NonNull<AvlTreeNode<T>>>,
    current_subtree: &'a Link<T>,
}

impl<'a, T: Ord + 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_subtree {
                None => {
                    match self.prev_nodes.pop() {
                        None => return None,
                        Some(ref prev_node) => {
                            unsafe {
                                let prev_node = prev_node.as_ptr() ;
                                self.current_subtree = &(*prev_node).right;
                                return Some(&(*prev_node).data);
                            }
                        }
                    }
                },
                Some(ref current_node) => {
                    unsafe {
                        if (*current_node.as_ptr()).left.is_some() {
                            self.prev_nodes.push(&current_node);
                            self.current_subtree = &(*current_node.as_ptr()).left;
                        }

                        if (*current_node.as_ptr()).right.is_some() {
                            self.current_subtree = &(*current_node.as_ptr()).right;
                            return Some(&(*current_node.as_ptr()).data)
                        }
                        self.current_subtree = &None;
                        return Some(&(*current_node.as_ptr()).data)
                    }
                }
            }
        }
    }
}

impl<'a, T: Ord + 'a> AvlTree<T> {
    fn iter(&'a self) -> Iter<T> {
        Iter {
            prev_nodes: Vec::new(),
            current_subtree: &self.root,
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
        
        let mut iter = tree.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
    }
}