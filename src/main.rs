mod lib;
use std::mem;
// use lib::binary_tree_unsafe;

pub fn main() {
    let mut st = String::from("dsadsa");
    let aa = &mut st as *mut String;
    unsafe {
        println!("{:#?}", *aa);
        *aa = String::from("xxxxxxxx");
        println!("{:#?}", *aa);
    }
}
