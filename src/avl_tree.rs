#![allow(unused)]
use std::cmp::{Ordering, max};
use std::iter::FromIterator;

type AvlTreeRef<T> = Option<Box<AvlTreeNode<T>>>;

#[derive(Debug, Clone, PartialEq)]
struct AvlTreeNode<T: Ord> {
    pub data: T,
    left: AvlTreeRef<T>,
    right: AvlTreeRef<T>,
    height: usize,
}

impl<T: Ord> AvlTreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            data: value,
            left: None,
            right: None,
            height: 0,
        }
    }

    pub fn set_left(&mut self, node: AvlTreeNode<T>) {
        self.left = Some(Box::new(node));
    }

    pub fn set_right(&mut self, node: AvlTreeNode<T>) {
        self.right = Some(Box::new(node));
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    fn right_height(&self) -> usize {
        self.left.as_ref().map_or(0, |right| right.height)
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
        let mut previous_nodes = Vec::<*mut AvlTreeNode<T>>::new();
        let mut current_subtree = &mut self.root;

        while let Some(node) = current_subtree {
            previous_nodes.push(&mut **node);
            match node.data.cmp(&value) {
                Ordering::Greater => current_subtree = &mut node.left,
                Ordering::Equal => return,
                Ordering::Less => current_subtree = &mut node.right,
            }
        }
        *current_subtree = Some(Box::new(AvlTreeNode::new(value)));
        for node_ptr in previous_nodes.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
        }
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
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => return None,
                    Some(ref prev_node) => {
                        self.current_tree = &prev_node.right;
                        return Some(&prev_node.data)
                    }
                },
                Some(ref current_node) => {
                    if current_node.left.is_some() {
                        self.prev_nodes.push(&current_node);
                        self.current_tree = &current_node.left;

                        continue;
                    }

                    if current_node.right.is_some() {
                        self.current_tree = &current_node.right;
                        return Some(&current_node.data)
                    }

                    self.current_tree = &None;
                    return Some(&current_node.data)
                }
            }
        }
    }
}

impl<T: Ord> FromIterator<T> for AvlTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = Self::new();

        for elem in iter {
            tree.insert(elem);
        }

        tree
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
                    height: 0,
                })),
                right: None,
                height: 1
            }))
        )
    }

    #[test]
    fn iterator() {
        let mut tree: AvlTree<u32> = AvlTree::new();
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        let mut iter = tree.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
    }
}