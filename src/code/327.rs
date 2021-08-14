use std::{cell::RefCell, rc::Rc};

fn main() {
    // playground
    //! https://leetcode.com/problems/count-of-range-sum/discuss/1378661/Rust-Divide-and-Conquer-with-prefix-array
    println!(
        "{}",
        Solution::count_range_sum(vec![-2, 5, -1], -2, -2) // 3
    );
}

pub struct Solution {}

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        Solution::count_sub_range_sum(
            Rc::new(RefCell::new(
                Some(0 as i64)
                    .iter()
                    .chain(nums.iter().map(|x| *x as i64).collect::<Vec<i64>>().iter())
                    .scan(0, |state, x| {
                        *state += x;
                        Some(*state as i64)
                    })
                    .collect::<Vec<i64>>(),
            )),
            0,
            nums.len(),
            lower as i64,
            upper as i64,
        )
    }

    fn count_sub_range_sum(
        prefix: Rc<RefCell<Vec<i64>>>,
        start: usize,
        end: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        if start >= end {
            return 0;
        }

        // Pre-compute the two halves, thus assuming both sorted
        let mid: usize = (start + end) / 2;
        let mut count: i32 =
            Self::count_sub_range_sum(Rc::clone(&prefix), start, mid, lower, upper)
                + Self::count_sub_range_sum(Rc::clone(&prefix), mid + 1, end, lower, upper);

        // loop over every pos in left and use two pointers for lower & upper
        let (mut i, mut j) = (mid + 1, mid + 1);
        for pos in start..=mid {
            // compute the range of acceptable values
            while i <= end && (*prefix.borrow())[i] - (*prefix.borrow())[pos] < lower {
                i += 1;
            }
            while j <= end && (*prefix.borrow())[j] - (*prefix.borrow())[pos] <= upper {
                j += 1;
            }
            count += (j - i) as i32;
        }

        // partially sort the vector
        let mut replacement = (*prefix.borrow())[start..=end].to_vec();
        replacement.sort();
        (*prefix.borrow_mut())[start..=end].copy_from_slice(&replacement[..]);

        count
    }
}
