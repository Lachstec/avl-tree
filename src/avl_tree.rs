#![allow(dead_code)]
use std::cmp::Ordering;
use std::mem;
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T: Ord> {
    value: T,
    left: Link<T>,
    right: Link<T>,

    height: usize,
}

impl<T: Ord> Node<T> {
    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height)
    }

    fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(self.left_height(), self.right_height())
    }

    fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }
 
    fn rotate_right(&mut self) -> bool {
        let left_node = match &self.left {
            None => return false,
            Some(_) => self.left.as_mut().unwrap(),
        };

        let left_right_subtree = left_node.right.take();
        let left_left_subtree = left_node.left.take();
        let mut new_right_subtree = mem::replace(&mut self.left, left_left_subtree);
        mem::swap(&mut self.value, &mut new_right_subtree.as_mut().unwrap().value);
        let right_tree = self.right.take();

        let new_right_node = new_right_subtree.as_mut().unwrap();
        new_right_node.left = left_right_subtree;
        new_right_node.right = right_tree;
        self.right = new_right_subtree;

        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }

    fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }

        let right_node = self.right.as_mut().unwrap();
        let right_left_tree = right_node.left.take();
        let right_right_tree = right_node.right.take();

        let mut new_left_tree = mem::replace(&mut self.right, right_right_tree);
        mem::swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().value);
        let left_tree = self.left.take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.right = right_left_tree;
        new_left_node.left = left_tree;
        self.left = new_left_tree;

        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }
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
        let mut prev_ptrs = Vec::<*mut Node<T>>::new();

        while let Some(current_node) = current_tree {
            prev_ptrs.push(&mut **current_node);
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
            height: 1,
        }));

        for ptr in prev_ptrs.into_iter().rev() {
            unsafe {(*ptr).update_height()}
        }

        true
    }
}

impl<'a, T: Ord + 'a> AvlTree<T> {
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.node_iter().map(|node| &node.value)
    }

    pub fn node_iter(&'a self) -> impl Iterator<Item = &'a Node<T>> + 'a {
        NodeIter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}

#[cfg(test)]
impl<T: Arbitrary + Ord> Arbitrary for AvlTree<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        let vec: Vec<T> = Arbitrary::arbitrary(g);
        vec.into_iter().collect()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let vec: Vec<T> = self.iter().cloned().collect();
        Box::new(vec.shrink().map(|val| val.into_iter().collect::<Self>()))
    }
}

pub struct Iter<'a, T: Ord> {
    prev_nodes: Vec<&'a Node<T>>,
    current_tree: &'a Link<T>,
}

pub struct NodeIter<'a, T: Ord> {
    prev_nodes: Vec<&'a Node<T>>,
    current_tree: &'a Link<T>,
}

impl<'a, T: Ord + 'a> Iterator for NodeIter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => return None,
                    Some(ref prev_node) => {
                        self.current_tree = &prev_node.right;
                        return Some(&prev_node);
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
                        return Some(&current_node);
                    }
                    self.current_tree = &None;
                    return Some(&current_node)
                }
            }
        }
    }
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
    use std::collections::BTreeSet;
    use quickcheck::TestResult;
    use itertools::equal;
    use quickcheck_macros::quickcheck;

    #[test]
    fn insert() {
        let mut tree = AvlTree::new();
        assert!(tree.insert(1));
        assert!(!tree.insert(1));
        assert!(tree.insert(2));

        assert_eq!(tree.root, Some(Box::new(
            Node {
                value: 1,
                height: 2,
                left: None,
                right: Some(Box::new(
                    Node {
                        value: 2,
                        left: None,
                        right: None,
                        height: 1,
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

    #[quickcheck]
    fn node_height(tree: AvlTree<u16>) -> bool {
        itertools::all(tree.node_iter(), |node| {
            node.height == 1 + std::cmp::max(node.left_height(), node.right_height())
        })
    }

    #[quickcheck]
    fn rotate_right(btree: BTreeSet<u8>) -> TestResult {
        let mut set = btree.iter().cloned().collect::<AvlTree<_>>();
        
        if !set.root.is_some() {
            return TestResult::discard();
        }

        if !set.root.as_mut().unwrap().rotate_right() {
            return TestResult::discard();
        }

        TestResult::from_bool(equal(set.iter(), btree.iter()))
    }

    #[quickcheck]
    fn rotate_right_balance_factor(data: Vec<u32>) -> TestResult {
        let mut set = data.iter().cloned().collect::<AvlTree<_>>();

        if !set.root.is_some() {
            return TestResult::discard();
        }

        let root = set.root.as_mut().unwrap();
        let balance_factor = root.balance_factor();

        if !root.rotate_right() {
            return TestResult::discard();
        }

        let tilted_factor = root.balance_factor();
        TestResult::from_bool(balance_factor - tilted_factor >= 2)
    }

    #[quickcheck]
    fn rotate_left_left_ident(tree: AvlTree<u8>) -> TestResult {
        if tree.root.is_none() {
            return TestResult::discard();
        }

        let mut rotated = tree.clone();
        let root = rotated.root.as_mut().unwrap();

        if root.rotate_left() {
            root.rotate_right();
        } else {
            root.rotate_right();
            root.rotate_left();
        }

        TestResult::from_bool(rotated == tree)
    }
}
