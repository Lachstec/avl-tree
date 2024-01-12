#![allow(dead_code)]
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T: Ord> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct AvlTree<T: Ord> {
    root: Link<T>,
}

impl<T: Ord> AvlTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            match current_node.value.cmp(&value) {
                Ordering::Greater => current_tree = &mut current_node.left,
                Ordering::Equal => return false,
                Ordering::Less => current_tree = &mut current_node.right,
            }
        }

        *current_tree = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));

        true
    }
}

impl<'a, T: Ord + 'a> AvlTree<T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    } 
}

pub struct Iter<'a, T: Ord> {
    prev_nodes: Vec<&'a Node<T>>,
    current_tree: &'a Link<T>,
}

impl<'a, T: Ord + 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => return None,
                    Some(ref prev_node) => {
                        self.current_tree = &prev_node.right;
                        return Some(&prev_node.value);
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
                        return Some(&current_node.value);
                    }
                    self.current_tree = &None;
                    return Some(&current_node.value)
                }
            }
        }
    }
}

impl<T: Ord> FromIterator<T> for AvlTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = Self::new();
        for value in iter {
            tree.insert(value);
        }
        tree
    }
}

#[cfg(test)]
mod avl_tree_tests {
    use super::*;

    #[test]
    fn insert() {
        let mut tree = AvlTree::new();
        assert!(tree.insert(1));
        assert!(!tree.insert(1));
        assert!(tree.insert(2));

        assert_eq!(tree.root, Some(Box::new(
            Node {
                value: 1,
                left: None,
                right: Some(Box::new(
                    Node {
                        value: 2,
                        left: None,
                        right: None,
                    }
                ))
            }
        )));
    }

    #[test]
    fn iter() {
        let mut tree = AvlTree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        for (expected, actual) in tree.iter().enumerate() {
            assert_eq!(&((expected + 1) as i32 ), actual)
        }
    }
}
