#![allow(dead_code)]
#![allow(unused_imports)]
extern crate rand;

use rand::Rng;
use std::alloc;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::mem;

#[derive(Debug, Clone)]
struct TreeNode<T>
where
    T: Ord + Clone,
{
    element: T,
    left: Option<*mut TreeNode<T>>,
    right: Option<*mut TreeNode<T>>,
}

#[derive(Debug, Clone)]
struct Tree<T>
where
    T: Ord + Clone,
{
    tree: Option<*mut TreeNode<T>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
{
    fn new(ele: T) -> TreeNode<T> {
        TreeNode {
            element: ele,
            left: Option::None,
            right: Option::None,
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
            *opetate = Option::Some(&mut TreeNode::new(word) as *mut TreeNode<T>);
            true
        }
    }

    fn alloc_mut_one() -> *mut Self {
        unsafe {
            let align = mem::align_of::<Self>();
            let layout = alloc::Layout::from_size_align(1, align).unwrap();
            alloc::alloc(layout) as *mut Self
        }
    }

    fn dealloc_mut_one(ptr: *mut Self) {
        unsafe {
            let align = mem::align_of::<Self>();
            let layout = alloc::Layout::from_size_align(1, align).unwrap();
            alloc::dealloc(ptr as *mut u8, layout);
        }
    }
}

impl<T> Tree<T>
where
    T: Ord + Clone,
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
                self.tree = Option::Some(&mut TreeNode::new(word) as *mut TreeNode<T>);
                true
            }
            Option::Some(x) => unsafe { (**x).insert(word) },
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

    println!("{:#?}", a.find_max());
}
