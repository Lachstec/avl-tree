#![allow(unused)]
use std::cmp::Ordering;

type AvlTreeRef<T> = Option<Box<AvlTreeNode<T>>>;

#[derive(Debug, Clone, PartialEq)]
struct AvlTreeNode<T: Ord> {
    pub data: T,
    left: AvlTreeRef<T>,
    right: AvlTreeRef<T>,
}

impl<T: Ord> AvlTreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            data: value,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, node: AvlTreeNode<T>) {
        self.left = Some(Box::new(node));
    }

    pub fn set_right(&mut self, node: AvlTreeNode<T>) {
        self.right = Some(Box::new(node));
    }
}

pub struct AvlTree<T: Ord> {
    root: AvlTreeRef<T>,
}

impl<T: Ord> AvlTree<T> {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn insert(&mut self, value: T) {
        let mut current_subtree = &mut self.root;

        while let Some(node) = current_subtree {
            match node.data.cmp(&value) {
                Ordering::Greater => current_subtree = &mut node.left,
                Ordering::Equal => return,
                Ordering::Less => current_subtree = &mut node.right,
            }
        }

        *current_subtree = Some(Box::new(AvlTreeNode::new(value)))
    }
}

impl<'a, T: 'a + Ord> AvlTree<T> {
    fn iter(&'a self) -> AvlTreeIterator<'a, T> {
        AvlTreeIterator {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}

pub struct AvlTreeIterator<'a, T: Ord> {
    prev_nodes: Vec<&'a AvlTreeNode<T>>,
    current_tree: &'a AvlTreeRef<T>
}

impl<'a, T: 'a + Ord> Iterator for AvlTreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn insert() {
        let mut tree: AvlTree<i32> = AvlTree::new();
        tree.insert(22);
        tree.insert(22);
        tree.insert(12);

        assert_eq!(
            tree.root,
            Some(Box::new(AvlTreeNode {
                data: 22,
                left: Some(Box::new(AvlTreeNode {
                    data: 12,
                    left: None,
                    right: None,
                })),
                right: None
            }))
        )
    }
}