//! Binary search tree. Build and walk.

use std::cmp::Ordering;

/// Represents binary search tree.
pub struct Tree<T: Copy + Ord> {
    root: Link<T>,
}

impl<T: Copy + Ord> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        insert_link(&mut self.root, value);
    }

    pub fn contains(&self, value: T) -> bool {
        contains_link(&self.root, value)
    }

    pub fn walk(&self) -> Vec<T> {
        let mut result = Vec::new();
        walk_inorder(&self.root, &mut result);
        result
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

/// Walks all descendants of link in order, producing sorted vector.
fn walk_inorder<T: Copy>(link: &Link<T>, acc: &mut Vec<T>) {
    link.as_ref().map(|node| {
        walk_inorder(&node.left, acc);
        acc.push(node.value);
        walk_inorder(&node.right, acc);
    });
}

type Link<T> = Option<Box<Node<T>>>;

fn new_link<T>(value: T) -> Link<T> {
    Some(Box::new(Node::new(value)))
}

fn insert_link<T: Ord>(link: &mut Link<T>, value: T) {
    match link {
        None => *link = new_link(value),
        Some(node) => match value.cmp(&node.value) {
            Ordering::Less => insert_link(&mut node.left, value),
            Ordering::Greater => insert_link(&mut node.right, value),
            Ordering::Equal => return,
        },
    }
}

fn contains_link<T: Ord>(link: &Link<T>, value: T) -> bool {
    match link {
        None => false,
        Some(node) => match value.cmp(&node.value) {
            Ordering::Less => contains_link(&node.left, value),
            Ordering::Greater => contains_link(&node.right, value),
            Ordering::Equal => true,
        },
    }
}

struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct IntoIter<T: Copy + Ord>(Tree<T>);

impl<T: Copy + Ord> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// Simple macro to create and fill a tree.
#[macro_export]
macro_rules! tree {
    ( $( $x:expr ), * ) => {
        {
            let mut t = Tree::new();
            $( t.insert($x); )*
            t
        }
    };
}
