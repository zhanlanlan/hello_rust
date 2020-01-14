fn so1() {
    /// 当元素数量为10的时候是有bug的，但是懒得改了，照着书上实现了一下。
    let mut str_ = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"];
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

fn so2() {}

pub fn run() {
    so1();
}
