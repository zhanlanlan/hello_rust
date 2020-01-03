#![allow(dead_code)]
#![allow(unused_imports)]
extern crate rand;

use rand::Rng;
use std::alloc;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt::Debug;
use std::mem;
use std::ptr;

#[derive(Debug, Clone)]
struct TreeNode<T>
where
    T: Debug + Ord + Clone,
{
    element: T,
    left: Option<*mut TreeNode<T>>,
    right: Option<*mut TreeNode<T>>,
}

#[derive(Debug, Clone)]
struct Tree<T>
where
    T: Debug + Ord + Clone,
{
    tree: Option<*mut TreeNode<T>>,
}

impl<T> Drop for Tree<T>
where
    T: Debug + Ord + Clone,
{
    fn drop(&mut self) {
        unsafe {
            if let Option::Some(x) = self.tree {
                (*x).drop_()
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Debug + Ord + Clone,
{
    fn new(ele: T) -> TreeNode<T> {
        TreeNode {
            element: ele,
            left: Option::None,
            right: Option::None,
        }
    }

    fn drop_(&mut self) {
        unsafe {
            match self.left {
                Option::Some(x) => (*x).drop_(),
                Option::None => (),
            }
            match self.right {
                Option::Some(x) => (*x).drop_(),
                Option::None => (),
            }
            println!("yeah awesome !");
            Self::dealloc_mut_one(self as *mut TreeNode<T>)
        }
    }

    fn find(&self, word: T) -> Option<TreeNode<T>> {
        if self.element == word {
            Option::Some(self.clone())
        } else {
            if let Option::Some(x) = if self.element < word {
                self.right
            } else {
                self.left
            } {
                unsafe { (*x).find(word) }
            } else {
                Option::None
            }
        }
    }

    fn find_min(&self) -> Option<TreeNode<T>> {
        match &self.left {
            Option::Some(x) => unsafe { (**x).find_min() },
            Option::None => Option::Some(self.clone()),
        }
    }

    fn find_max(&self) -> Option<TreeNode<T>> {
        match &self.right {
            Option::Some(x) => unsafe { (**x).find_min() },
            Option::None => Option::Some(self.clone()),
        }
    }

    fn insert(&mut self, word: T) -> bool {
        let opetate = if self.element > word {
            &mut self.left
        } else {
            &mut self.right
        };

        if let Option::Some(x) = opetate {
            unsafe { (**x).insert(word) }
        } else {
            let val = Self::alloc_mut_one(word);
            *opetate = Option::Some(val);
            true
        }
    }

    fn alloc_mut_one(word: T) -> *mut TreeNode<T> {
        unsafe {
            let layout = alloc::Layout::new::<TreeNode<T>>();
            let ptr_ = alloc::alloc(layout) as *mut TreeNode<T>;
            (*ptr_).element = word;
            (*ptr_).left = Option::None;
            (*ptr_).right = Option::None;
            ptr_
        }
    }

    fn dealloc_mut_one(ptr: *mut Self) {
        unsafe {
            let layout = alloc::Layout::new::<TreeNode<T>>();
            alloc::dealloc(ptr as *mut u8, layout);
        }
    }
}

impl<T> Tree<T>
where
    T: Debug + Ord + Clone,
{
    fn new() -> Tree<T> {
        Tree { tree: Option::None }
    }

    fn find(&self, word: T) -> Option<TreeNode<T>> {
        match &self.tree {
            Option::Some(ref x) => unsafe { (**x).find(word) },
            Option::None => Option::None,
        }
    }

    fn find_min(&self) -> Option<TreeNode<T>> {
        match &self.tree {
            Option::Some(x) => unsafe { (**x).find_min() },
            Option::None => Option::None,
        }
    }

    fn find_max(&self) -> Option<TreeNode<T>> {
        match &self.tree {
            Option::Some(x) => unsafe { (**x).find_max() },
            Option::None => Option::None,
        }
    }

    fn insert(&mut self, word: T) -> bool {
        match &mut self.tree {
            Option::None => {
                self.tree = Option::Some(TreeNode::alloc_mut_one(word));
                true
            }
            Option::Some(x) => unsafe { (**x).insert(word) },
        }
    }
}

pub fn run() {
    let mut rng = rand::thread_rng();
    let mut t = Tree::<u64>::new();
    for _ in 0..1000 {
        t.insert(rng.gen_range(2345, 30000000));
    }
    t.insert(100);
    t.insert(101);
    t.insert(99);

    println!("{:#?}", t.find(101));
}
