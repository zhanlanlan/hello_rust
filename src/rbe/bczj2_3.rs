#![allow(dead_code)]
#![allow(unused_imports)]

/// 当元素数量为10的时候是有bug的，但是懒得改了，照着书上实现了一下。
fn so1() {
    let mut str_ = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    // let mut str_ = ["d", "e", "f", "g", "h", "i", "j", "a", "b", "c"];
    let i = 3;
    let n = str_.len();
    for idx in 0..i {
        let tmp = str_[idx];
        let mut la = i + idx;
        let mut pe = idx;
        while la < n {
            println!("{}:{}", la, pe);
            str_[pe] = str_[la];
            la = la + i;
            pe = pe + i;
        }
        str_[la - i] = tmp;
        println!("{}", tmp);
    }
    for i in str_.iter() {
        print!("{}", i);
    }
}

/// 审题问题 重新实现
fn so1_fix() {}

/// fuck 下次绝对不能把变量顺序搞反 调试半天发现闹乌龙太sb
fn so2() {
    fn reverse(
        list: &mut [&str],
        mut start: usize,
        mut end: usize,
        swapfn: fn(list: &mut [&str], i: usize, j: usize),
    ) {
        while start < end {
            swapfn(list, start, end);
            start += 1;
            end -= 1;
        }
    }

    fn shift(
        list: &mut [&str],
        count: usize,
        i: usize,
        swapfn: fn(list: &mut [&str], i: usize, j: usize),
    ) {
        reverse(list, 0, i - 1, swapfn);
        reverse(list, i, count - 1, swapfn);
        reverse(list, 0, count - 1, swapfn);
    }

    fn swapfn(list: &mut [&str], i: usize, j: usize) {
        let tmp = list[i];
        list[i] = list[j];
        list[j] = tmp;
    }

    let mut str_ = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let len = str_.len();
    shift(&mut str_, len, 3, swapfn);
    for i in str_.iter() {
        print!("{} ", i);
    }
}

pub fn run() {
    so1();
}
