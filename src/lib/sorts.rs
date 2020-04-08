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

// 为什么不允许加减法越界只允许数组下标越界
fn quick_sort(mut nums: Vec<i64>) -> Vec<i64> {
    fn sort(mut nums: Vec<i64>, l: usize, r: usize) -> Vec<i64> {
        if l < r {
            let (mut xx, q) = partition(nums, l, r);
            xx = sort(xx, l, q - 1);
            xx = sort(xx, q + 1, r);
            nums = xx
        }
        nums
    };

    fn partition(mut nums: Vec<i64>, l: usize, r: usize) -> (Vec<i64>, usize) {
        let x = nums[r];
        let mut i = l.wrapping_sub(1);
        for j in l..r {
            if nums[j] <= x {
                i = i.wrapping_add(1);
                nums.swap(i, j);
            }
        }
        nums.swap(i + 1, r);
        (nums, i + 1)
    };

    let len = nums.len();
    sort(nums, 0, len - 1)
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
                2, 32, 14, 21, 5, 43, 365, 66788, 437, 54, 7657, 65, 532, 43, 2
            ])
        )
    }
}
