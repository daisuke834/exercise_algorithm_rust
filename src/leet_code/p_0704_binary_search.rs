// copyright (c) 2022 Daisuke Hashimoto
// 704. Binary Search
// https://leetcode.com/problems/binary-search/

use std::cmp::Ordering;

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut b = 0;
        let mut e = n;
        let mut result = -1;
        while b < e {
            let m = (b + e) / 2;
            match nums[m].cmp(&target) {
                Ordering::Equal => {
                    result = m as i32;
                    break;
                }
                Ordering::Less => {
                    b = m + 1;
                }
                Ordering::Greater => {
                    e = m;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum() {
        assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
        assert_eq!(0, Solution::search(vec![5], 5));
        assert_eq!(-1, Solution::search(vec![5], 7));
    }
}
