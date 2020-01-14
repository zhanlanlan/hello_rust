use rand::Rng;
use std::collections::hash_set;

fn gene() {
    let mut rng = rand::thread_rng();
    let mut list_: [u64; 100000] = [0; 100000];
    for i in 0..100000 {
        list_[i] = i as u64;
    }
    let mut res = hash_set::HashSet::<u64>::new();

    for i in 0..90000 {
        let rand = rng.gen_range(0, 100000);
        let tmp = list_[i];
        list_[i] = list_[rand];
        list_[rand] = tmp;
        res.insert(tmp);
    }

    // for i in list_.into_iter() {
    //     res.insert(*i);
    // }

    println!("{}", res.len());
}

pub fn run() {
    gene();
}
