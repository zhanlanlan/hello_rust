pub fn run() {
    unimplemented!();
}

fn bubble_sort(mut nums: Vec<i64>) -> Vec<i64> {
    for i in 0..nums.len() {
        for j in 0..nums.len() - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
    nums
}

fn insert_sort(mut nums: Vec<i64>) -> Vec<i64> {
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
    nums
}

fn quick_sort(mut nums: Vec<i64>) -> Vec<i64> {
    fn sort(mut nums: Vec<i64>, l: isize, r: isize) -> Vec<i64> {
        if l < r {
            let (mut xx, q) = partition(nums, l, r);
            xx = sort(xx, l, q - 1);
            xx = sort(xx, q + 1, r);
            nums = xx
        }
        nums
    };

    fn partition(mut nums: Vec<i64>, l: isize, r: isize) -> (Vec<i64>, isize) {
        let x = nums[r as usize];
        let mut i = l - 1;
        for j in l..r {
            if nums[j as usize] <= x {
                i = i + 1;
                nums.swap(i as usize, j as usize);
            }
        }
        nums.swap((i + 1) as usize, r as usize);
        (nums, i + 1)
    };

    let len = nums.len();
    sort(nums, 0, (len - 1) as isize)
}

mod test {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        println!(
            "{:?}",
            bubble_sort(vec![
                2, 32, 14, 21, 5, 43, 365, 66788, 437, 54, 7657, 65, 532, 43, 2
            ])
        )
    }

    #[test]
    fn test_insert_sort() {
        println!(
            "{:?}",
            insert_sort(vec![
                2, 32, 14, 21, 5, 43, 365, 66788, 437, 54, 7657, 65, 532, 43, 2, 99999, 0
            ])
        )
    }

    #[test]
    fn test_quick_sort() {
        println!(
            "{:?}",
            quick_sort(vec![
                2, 32, 14, 21, 5, 43, 365, 66788, 437, 54, 7657, 65, 532, 43, 2, 0, 0, 0, 0
            ])
        )
    }
}
