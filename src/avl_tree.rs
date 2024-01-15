#![allow(dead_code)]
use std::cmp::Ordering;
use std::ptr::NonNull;
use std::mem;
use std::default::Default;

/// Represents a single node in an avl tree
#[derive(Debug, Clone, PartialEq)]
pub struct Node<T: Ord> {
    /// value stored in the node
    value: T,
    /// left subtree connected to this node
    left: Link<T>,
    /// right subtree connected to this node
    right: Link<T>,
    /// height of the node
    height: usize,
}

impl<T: Ord> Node<T> {
    /// Retrieves the height of the left subtree if it exists, else returns 0.
    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| unsafe { (*left.as_ptr()).height })
    }

    /// Retrieves the height of the right subtree if it exists, else returns 0.
    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| unsafe { (*right.as_ptr()).height })
    }

    /// Updates the height of a node by setting it equal to 1 + the greater height of 
    /// its children.
    fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(self.left_height(), self.right_height())
    }

    /// Computes the balance factor as defined for an [avl tree](https://en.wikipedia.org/wiki/AVL_tree#Definition).
    fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }
 
    /// Performs a right rotation of the current node and its children as specified for avl trees.
    fn rotate_right(&mut self) -> bool {
        let left_node = match &self.left {
            None => return false,
            Some(_) => *self.left.as_mut().unwrap(),
        };
        unsafe {
            let left_right_subtree = (*left_node.as_ptr()).right.take();
            let left_left_subtree = (*left_node.as_ptr()).left.take();
            let mut new_right_subtree = mem::replace(&mut self.left, left_left_subtree);
            mem::swap(&mut self.value, &mut (*new_right_subtree.as_mut().unwrap().as_ptr()).value);
            let right_tree = self.right.take();

            let new_right_node = new_right_subtree.as_mut().unwrap();
            (*new_right_node.as_ptr()).left = left_right_subtree;
            (*new_right_node.as_ptr()).right = right_tree;
            self.right = new_right_subtree;

            if let Some(node) = self.right.as_mut() {
                (*node.as_ptr()).update_height();
            }
        }

        self.update_height();

        true
    }

    /// Performs a left rotation of this node and its children as specified for avl trees.
    fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }
        unsafe {
            let right_node = self.right.as_mut().unwrap();
            let right_left_tree = (*right_node.as_ptr()).left.take();
            let right_right_tree = (*right_node.as_ptr()).right.take();

            let mut new_left_tree = mem::replace(&mut self.right, right_right_tree);
            mem::swap(&mut self.value, &mut (*new_left_tree.as_mut().unwrap().as_ptr()).value); 
            let left_tree = self.left.take();

            let new_left_node = *new_left_tree.as_mut().unwrap();
            (*new_left_node.as_ptr()).right = right_left_tree;
            (*new_left_node.as_ptr()).left = left_tree;
            self.left = new_left_tree;

            if let Some(node) = self.left.as_mut() {
                (*node.as_ptr()).update_height();
            }
        }
        self.update_height();

        true
    }

    /// Rebalances the current node to restore the avl critirium after an insertion.
    fn rebalance(&mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                // currently node is right-heavy
                let right_node = *self.right.as_mut().unwrap();
                
                unsafe {
                    // inner node is currently left-heavy
                    if (*right_node.as_ptr()).balance_factor() == 1 {
                        (*right_node.as_ptr()).rotate_right();
                    }
                }

                self.rotate_left();
                true
            },
            2 => {
                // currently node is left-heavy
                let left_node = *self.left.as_mut().unwrap();
                unsafe {
                    // inner node is currentyl right-heavy
                    if (*left_node.as_ptr()).balance_factor() == -1 {
                        (*left_node.as_ptr()).rotate_left();
                    }
                }

                self.rotate_right();

                true
            },
            _ => false
        }
    }
}

/// A link between nodes in a tree.
type Link<T> = Option<NonNull<Node<T>>>;

/// Generic AvlTree implementation that permits no duplicate entries.
#[derive(Debug, Clone, PartialEq)]
pub struct AvlTree<T: Ord> {
    root: Link<T>,
}

impl<T: Ord> AvlTree<T> {
    /// Create a new AvlTree instance
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    /// Try to insert the value into the tree. Returns true on success, else false.
    /// 
    /// ## Arguments
    /// * `value` - Value to insert into the tree
    pub fn insert(&mut self, value: T) -> bool {
        let mut current_tree = &mut self.root;
        let mut prev_ptrs = Vec::<*mut Node<T>>::new();
        unsafe  {
            while let Some(current_node) = current_tree {
                prev_ptrs.push(current_node.as_ptr());
                match (*current_node.as_ptr()).value.cmp(&value) {
                    Ordering::Greater => current_tree = &mut (*current_node.as_ptr()).left,
                    Ordering::Equal => return false,
                    Ordering::Less => current_tree = &mut (*current_node.as_ptr()).right,
                }
            }
        }
        unsafe {
            *current_tree = Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                left: None,
                right: None,
                height: 1,
            }))));
        }

        for ptr in prev_ptrs.into_iter().rev() {
            unsafe {
                let node = &mut *ptr;
                node.update_height();
                node.rebalance();
            }
        }

        true
    }

    /// Checks if the AvlTree contains the value T.
    /// 
    /// ## Arguments
    /// * `value` The value to check
    /// ## Returns
    /// `true`, when `value` is in the AvlTree, else `false``.
    pub fn contains(&self, value: &T) -> bool {
        let mut current_tree = &self.root;
        while let Some(node) = current_tree {
            unsafe {
                match (*node.as_ptr()).value.cmp(value) {
                    Ordering::Greater => current_tree = &(*node.as_ptr()).left,
                    Ordering::Equal => return true,
                    Ordering::Less => current_tree = &(*node.as_ptr()).right,
                }
            }
        }
        false
    }

    /// Return the number of elements in the AvlTree.
    pub fn len(&self) -> usize {
        self.iter().count()
    }
}

impl<'a, T: Ord + 'a> AvlTree<T> {
    /// Returns an iterator over the values in the tree. 
    /// The iterator performs an in-order depth traversal of the tree.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.node_iter().map(|node| &node.value)
    }

    /// Returns an iterator over the actual nodes in the tree.
    /// The iterator performs an in-order depth traversal of the tree.
    pub fn node_iter(&'a self) -> impl Iterator<Item = &'a Node<T>> + 'a {
        NodeIter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}

impl<T: Ord> Drop for AvlTree<T> {
    fn drop(&mut self) {
        if self.root.is_none() {
            return;
        }
        let mut stack = Vec::new();
        let mut curr_node = self.root.unwrap();
        let mut nodes = Vec::new();

        stack.push(curr_node);
        while !stack.is_empty() {
            curr_node = stack.pop().unwrap();
            nodes.push(curr_node);
            unsafe {
                if (*curr_node.as_ptr()).right.is_some() {
                    stack.push((*curr_node.as_ptr()).right.unwrap());
                }
                if (*curr_node.as_ptr()).left.is_some() {
                    stack.push((*curr_node.as_ptr()).left.unwrap());
                }
            }
        }

        for node in nodes {
            unsafe { let _box = Box::from_raw(node.as_ptr()); }
        }
    }
}

impl<T: Ord> Default for AvlTree<T> {
    fn default() -> Self {
        Self {
            root: None,
        }
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
                    unsafe {
                        if (*current_node.as_ptr()).left.is_some() {
                            self.prev_nodes.push(&(*current_node.as_ptr()));
                            self.current_tree = &(*current_node.as_ptr()).left;

                            continue;
                        }
                        if (*current_node.as_ptr()).right.is_some() {
                            self.current_tree = &(*current_node.as_ptr()).right;
                            return Some(&(*current_node.as_ptr()));
                        }
                        self.current_tree = &None;
                        return Some(&(*current_node.as_ptr()))
                    }
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
                    unsafe {
                        if (*current_node.as_ptr()).left.is_some() {
                            self.prev_nodes.push(&(*current_node.as_ptr()));
                            self.current_tree = &(*current_node.as_ptr()).left;

                            continue;
                        }
                        if (*current_node.as_ptr()).right.is_some() {
                            self.current_tree = &(*current_node.as_ptr()).right;
                            return Some(&(*current_node.as_ptr()).value);
                        }
                        self.current_tree = &None;
                        return Some(&(*current_node.as_ptr()).value)
                    }
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
    use itertools::Itertools;
    use rand::Rng;
    use std::collections::BTreeSet;

    #[test]
    fn insert_iter() {
        let mut tree = AvlTree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        for (expected, actual) in tree.iter().enumerate() {
            assert_eq!(&((expected + 1) as i32 ), actual)
        }
    }

    #[test]
    fn node_height() {
        let mut tree = AvlTree::new();
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            tree.insert(rng.gen::<u32>());
        }
        assert!(itertools::all(tree.node_iter(), |node| {
            node.height == 1 + std::cmp::max(node.left_height(), node.right_height())
        }));
    }

    #[test]
    fn traversal_order() {
        let mut tree = AvlTree::new();
        let mut expected = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let num = rng.gen::<u32>();
            tree.insert(num);
            expected.push(num);
        }
        assert!(itertools::equal(expected.iter().sorted(), tree.iter()));
    }

    #[test]
    fn balanced_nodes() {
        let mut tree = AvlTree::new();
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            tree.insert(rng.gen::<u32>());
        }
        assert!(itertools::all(tree.node_iter(), |node| node.balance_factor().abs() < 2));
    }

    #[test]
    fn contains_parity() {
        let mut tree = AvlTree::new();
        let mut expected = BTreeSet::new();
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let num = rng.gen::<u32>();
            tree.insert(num);
            expected.insert(num);
        }
        assert!(itertools::all(expected.iter(), |value| tree.contains(value) == expected.contains(value)))
    }

    #[test]
    fn tree_length() {
        let mut tree = AvlTree::new();
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            tree.insert(rng.gen::<u32>());
        }
        assert_eq!(1000, tree.len())
    }
}
