#![allow(dead_code)]

extern crate rand;

use rand::RngCore;

use self::rand::Rng;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::mem;

#[derive(Debug, Clone, PartialEq)]
struct TreeNode<T>
where
    T: PartialOrd,
{
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug, Clone)]
struct Tree<T>
where
    T: PartialOrd,
{
    tree: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: PartialOrd,
{
    fn new(ele: T) -> TreeNode<T> {
        TreeNode {
            element: ele,
            left: Option::None,
            right: Option::None,
        }
    }

    fn find(&self, word: T) -> Option<&TreeNode<T>> {
        if self.element == word {
            Option::Some(&self)
        } else {
            if let Option::Some(ref x) = if self.element < word {
                &self.right
            } else {
                &self.left
            } {
                x.find(word)
            } else {
                Option::None
            }
        }
    }

    fn find_min(&self) -> Option<&TreeNode<T>> {
        match &self.left {
            Option::Some(x) => x.find_min(),
            Option::None => Option::Some(&self),
        }
    }

    fn find_max(&self) -> Option<&TreeNode<T>> {
        match &self.right {
            Option::Some(x) => x.find_max(),
            Option::None => Option::Some(&self),
        }
    }

    fn insert(&mut self, word: T) -> bool {
        let operate = if self.element > word {
            &mut self.left
        } else {
            &mut self.right
        };
        if let Option::Some(x) = operate {
            (*x).insert(word)
        } else {
            *operate = Option::Some(Box::new(TreeNode::new(word)));
            true
        }
    }

    fn delete(mut self, word: T) -> bool {
        //        if self.element < word {
        //            if let Some()
        //        }
        true
    }
}

impl<T> Tree<T>
where
    T: PartialOrd,
{
    fn new() -> Tree<T> {
        Tree { tree: Option::None }
    }

    fn find(&self, word: T) -> Option<&TreeNode<T>> {
        match &self.tree {
            Option::Some(ref x) => x.find(word),
            Option::None => Option::None,
        }
    }

    fn find_min(&self) -> Option<&TreeNode<T>> {
        match &self.tree {
            Option::Some(x) => x.find_min(),
            Option::None => Option::None,
        }
    }

    fn find_max(&self) -> Option<&TreeNode<T>> {
        match &self.tree {
            Option::Some(x) => x.find_max(),
            Option::None => Option::None,
        }
    }

    fn insert(&mut self, word: T) -> bool {
        match &mut self.tree {
            Option::None => {
                self.tree = Option::Some(Box::from(TreeNode::new(word)));
                true
            }
            Option::Some(x) => x.insert(word),
        }
    }
}

pub fn run() {
    let mut rng = rand::thread_rng();
    let mut a = Tree::<u64>::new();
    for _ in 0..3 {
        a.insert(rng.gen_range(2345, 8765));
    }
    a.insert(100);
    a.insert(101);
    a.insert(99);

    println!("{:#?}", a);
}
