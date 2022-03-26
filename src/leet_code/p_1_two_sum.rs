// copyright (c) 2022 Daisuke Hashimoto
// LeetCode 1. Two Sum.
// https://leetcode.com/problems/two-sum/
// Given an array of integers nums and an integer target,
// return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.
// You can return the answer in any order.

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let solution = Vec::new();
        let n = nums.len();
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}
