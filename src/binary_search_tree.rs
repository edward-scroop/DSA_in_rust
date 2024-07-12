use std::rc::Rc;
use std::{cmp, mem};

#[derive(Debug)]
pub struct Error {
    details: String,
}

#[allow(dead_code)]
impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

#[allow(dead_code)]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

#[allow(dead_code)]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(PartialEq, Debug)]
pub struct BinarySearchTree<T> {
    root: Option<Rc<BinarySearchTreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTreeNode<T> {
    key: isize,
    data: Option<T>,
    left_child: Option<Rc<BinarySearchTreeNode<T>>>,
    right_child: Option<Rc<BinarySearchTreeNode<T>>>,
}

impl<T> PartialEq for BinarySearchTreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for BinarySearchTreeNode<T> {}

impl<T> PartialOrd for BinarySearchTreeNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.key.cmp(&other.key))
    }
}

impl<T> Ord for BinarySearchTreeNode<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<T: Copy> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, key: isize, data: T) -> Result<(), Error> {
        match self.root {
            Some(ref mut root_node) => root_node.insert(key, data),
            None => Ok({
                let new_node = Rc::new(BinarySearchTreeNode {
                    key,
                    data: Some(data),
                    left_child: None,
                    right_child: None,
                });
                self.root = Some(new_node);
            }),
        }
    }

    pub fn remove(&mut self, key: isize) -> Result<T, Error> {
        match self.root {
            Some(ref mut root_node) => root_node.remove(root_node, key),
            None => Err(Error::new("tree is empty")),
        }
    }

    pub fn search(&mut self, key: isize) -> Result<T, Error> {
        match self.root {
            Some(ref root_node) => root_node.search(key),
            None => Err(Error::new("Tree is empty")),
        }
    }

    pub fn depth(&self, key: isize) -> Result<usize, Error> {
        let root_node = self.root.as_ref().unwrap();
        let depth: Result<usize, Error> = BinarySearchTreeNode::depth(root_node, key);

        depth
    }

    pub fn height(&self) -> Result<usize, Error> {
        if let Some(ref root_node) = self.root {
            Ok(root_node.height())
        } else {
            Err(Error::new("Tree is empty"))
        }
    }
}

impl<T: Clone> BinarySearchTreeNode<T> {
    fn insert(&mut self, key: isize, data: T) -> Result<(), Error> {
        match self.key.cmp(&key) {
            cmp::Ordering::Greater => match self.right_child {
                Some(ref mut next_node) => next_node.insert(key, data),
                None => Ok({
                    self.right_child = Some(Rc::new(BinarySearchTreeNode {
                        key,
                        data: Some(data),
                        left_child: None,
                        right_child: None,
                    }));
                }),
            },
            cmp::Ordering::Less => match self.left_child {
                Some(ref mut next_node) => next_node.insert(key, data),
                None => Ok({
                    self.left_child = Some(Rc::new(BinarySearchTreeNode {
                        key,
                        data: Some(data),
                        left_child: None,
                        right_child: None,
                    }));
                }),
            },
            cmp::Ordering::Equal => Err(Error::new("key already exists")),
        }
    }

    fn remove(&mut self, previous_node: &mut BinarySearchTreeNode<T>, key: isize) -> Result<T, Error> {
        match self.key.cmp(&key) {
            cmp::Ordering::Equal => {
                let data = self.data;
                if self.right_child.is_some() {
                    if previous_node.right_child.unwrap().key == key {
                        previous_node.right_child = self.right_child;
                    } else if previous_node.left_child.unwrap().key == key {
                        previous_node.left_child = self.right_child;
                    }
                } else if self.left_child.is_some() {
                    if previous_node.right_child.unwrap().key == key {
                        previous_node.right_child = self.left_child;
                    } else if previous_node.left_child.unwrap().key == key {
                        previous_node.left_child = self.left_child;
                    }
                } else {
                    if previous_node.right_child.unwrap().key == key {
                        previous_node.right_child = None;
                    } else if previous_node.left_child.unwrap().key == key {
                        previous_node.left_child = None;
                    }
                }
                Ok(data.unwrap())
            },
            cmp::Ordering::Greater => match self.right_child {
                Some(ref mut next_node) => {
                    next_node.remove(self, key)
                }
                None => Err(Error::new("key not found")),
            },
            cmp::Ordering::Less => match self.left_child {
                Some(ref mut next_node) => {
                    next_node.remove(self, key)
                }
                None => Err(Error::new("key not found")),
            },
        }
    }

    fn search(&self, key: isize) -> Result<T, Error> {
        match self.key.cmp(&key) {
            cmp::Ordering::Equal => Ok(self.data.clone()),
            cmp::Ordering::Greater => {
                if self.left_child != None {
                    self.left_child.search(key)
                } else {
                    Error("Key does not exist")
                }
            },
            cmp::Ordering::Less => {
                if self.right_child != None {
                    self.right_child.search(key)
                } else {
                    Error("Key does not exist")
                }
            },
        }
    }

    fn find_successor(&mut self) -> BinarySearchTreeNode<T> {
        if self.left_child.is_some() {
            let next_node = self.left_child.as_mut().unwrap();
            let successor = next_node.find_successor();
            if next_node.left_child.is_none() {
                self.left_child = next_node.right_child.take()
            }
            successor
        } else {
            mem::replace(
                self,
                BinarySearchTreeNode {
                    key: 0,
                    data: None,
                    right_child: None,
                    left_child: None,
                },
            )
        }
    }

    fn depth(node: &Self, key: isize) -> Result<usize, Error> {
        let mut depth: Result<usize, Error> = Ok(0);
        match node.key.cmp(&key) {
            cmp::Ordering::Greater => match node.left_child {
                Some(ref next_node) => {
                    depth = BinarySearchTreeNode::depth(next_node.as_ref(), key);
                    if let Ok(returned_depth) = depth {
                        Ok(depth.unwrap() + 1)
                    } else {
                        depth
                    }
                }
                None => Err(Error::new("key not found")),
            },
            cmp::Ordering::Less => match node.right_child {
                Some(ref next_node) => {
                    depth = BinarySearchTreeNode::depth(next_node.as_ref(), key);
                    if let Ok(returned_depth) = depth {
                        Ok(depth.unwrap() + 1)
                    } else {
                        depth
                    }
                }
                None => Err(Error::new("key not found")),
            },
            cmp::Ordering::Equal => depth,
        }
    }

    fn height(&self) -> usize {
        let (mut right_depth, mut left_depth) = (0usize, 0usize);

        if let Some(ref next_node) = self.right_child {
            right_depth = next_node.height() + 1;
        }
        if let Some(ref next_node) = self.left_child {
            left_depth = next_node.height() + 1;
        }

        if right_depth > left_depth {
            right_depth
        } else {
            left_depth
        }
    }
}

#[cfg(test)]
mod binary_search_tree_tests {
    use super::*;

    #[test]
    fn test_bst_insert() {
        let true_tree = BinarySearchTree {
            root: {
                Some(Box::new(BinarySearchTreeNode {
                    key: 5isize,
                    data: Some("5"),
                    left_child: Some(Box::new(BinarySearchTreeNode {
                        key: 1isize,
                        data: Some("1"),
                        left_child: None,
                        right_child: None,
                    })),
                    right_child: Some(Box::new(BinarySearchTreeNode {
                        key: 10isize,
                        data: Some("10"),
                        left_child: None,
                        right_child: Some(Box::new(BinarySearchTreeNode {
                            key: 100isize,
                            data: Some("degenerate tree"),
                            left_child: None,
                            right_child: None,
                        })),
                    })),
                }))
            },
        };
        let mut test_tree = BinarySearchTree::new();
        test_tree.insert(5isize, "5");
        test_tree.insert(10isize, "10");
        test_tree.insert(1isize, "1");
        test_tree.insert(100isize, "degenerate tree");
        assert_eq!(test_tree, true_tree, "binary tree insert not as expected");
        let true_tree = BinarySearchTree {
            root: {
                Some(Box::new(BinarySearchTreeNode {
                    key: 99isize,
                    data: Some(99),
                    left_child: Some(Box::new(BinarySearchTreeNode {
                        key: 12isize,
                        data: Some(12),
                        left_child: None,
                        right_child: None,
                    })),
                    right_child: None,
                }))
            },
        };
        let mut test_tree = BinarySearchTree::new();
        test_tree.insert(99isize, 99);
        test_tree.insert(12isize, 12);
        assert_eq!(test_tree, true_tree, "binary tree insert not as expected");
        assert!(
            test_tree.insert(12isize, 23535).is_err(),
            "inserted key that already exists"
        );
    }

    #[test]
    fn test_bst_remove() {
        let true_tree = BinarySearchTree {
            root: {
                Some(Box::new(BinarySearchTreeNode {
                    key: 100isize,
                    data: Some("degenerate tree"),
                    left_child: None,
                    right_child: None,
                }))
            },
        };
        let mut test_tree = BinarySearchTree {
            root: {
                Some(Box::new(BinarySearchTreeNode {
                    key: 5isize,
                    data: Some("5"),
                    left_child: Some(Box::new(BinarySearchTreeNode {
                        key: 1isize,
                        data: Some("1"),
                        left_child: None,
                        right_child: None,
                    })),
                    right_child: Some(Box::new(BinarySearchTreeNode {
                        key: 10isize,
                        data: Some("10"),
                        left_child: None,
                        right_child: Some(Box::new(BinarySearchTreeNode {
                            key: 100isize,
                            data: Some("degenerate tree"),
                            left_child: None,
                            right_child: None,
                        })),
                    })),
                }))
            },
        };
        let _ = test_tree.remove(5isize);
        let _ = test_tree.remove(10isize);
        let _ = test_tree.remove(1isize);
        assert_eq!(test_tree, true_tree, "binary tree remove not as expected");
        assert!(
            test_tree.remove(5isize).is_err(),
            "removed value that doesn't exist"
        );
    }

    #[test]
    fn test_bst_depth() {
        let true_tree = BinarySearchTree {
            root: {
                Some(Box::new(BinarySearchTreeNode {
                    key: 5isize,
                    data: Some("5"),
                    left_child: Some(Box::new(BinarySearchTreeNode {
                        key: 1isize,
                        data: Some("1"),
                        left_child: None,
                        right_child: None,
                    })),
                    right_child: Some(Box::new(BinarySearchTreeNode {
                        key: 10isize,
                        data: Some("10"),
                        left_child: None,
                        right_child: Some(Box::new(BinarySearchTreeNode {
                            key: 100isize,
                            data: Some("degenerate tree"),
                            left_child: None,
                            right_child: None,
                        })),
                    })),
                }))
            },
        };
        assert_eq!(
            true_tree.depth(100).unwrap(),
            2,
            "binary tree depth not as expected"
        );
        assert_eq!(
            true_tree.depth(5).unwrap(),
            0,
            "binary tree depth not as expected"
        );
        assert_eq!(
            true_tree.depth(10).unwrap(),
            1,
            "binary tree depth not as expected"
        );
        assert!(
            true_tree.depth(9999).is_err(),
            "found depth of non existent key"
        );
    }
}
