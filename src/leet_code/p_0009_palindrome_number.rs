// copyright (c) 2022 Daisuke Hashimoto
// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/
// Given an integer x, return true if x is palindrome integer.
// An integer is a palindrome when it reads the same backward as forward.
// For example, 121 is a palindrome while 123 is not.

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();
        let n = x_str.len();
        let mut result = true;
        for i in 0..n {
            if x_str.chars().nth(i) != x_str.chars().nth(n - i - 1) {
                result = false;
                break;
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
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(-121));
        assert_eq!(false, Solution::is_palindrome(10));
        assert_eq!(true, Solution::is_palindrome(111111111));
        assert_eq!(false, Solution::is_palindrome(111111211));
        assert_eq!(false, Solution::is_palindrome(-111111111));
    }
}
