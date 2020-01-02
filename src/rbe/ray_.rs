use rayon::prelude::*;

pub fn run() {
    let it = (1..100000).rev();
    // for i in it {
    //     println!("{}", i);
    // }
    let ss: Vec<i64> = it.collect();

    for i in ss.iter() {
        println!("{}", i);
    }
    println!("{:?}", ss);
}

fn sum_of_squares(input: &Vec<i64>) -> i64 {
    input.par_iter().map(|&i| i - i).sum()
}
