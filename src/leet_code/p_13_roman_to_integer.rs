// copyright (c) 2022 Daisuke Hashimoto
// 13. Roman to Integer
// https://leetcode.com/problems/roman-to-integer/

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn char_to_int(c: Option<char>) -> Option<i32> {
        match c {
            Some('I') => Some(1),
            Some('V') => Some(5),
            Some('X') => Some(10),
            Some('L') => Some(50),
            Some('C') => Some(100),
            Some('D') => Some(500),
            Some('M') => Some(1000),
            Some(_) => None,
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let n = s.len();
        let mut result = 0;
        for i in 0..n {
            let d = Solution::char_to_int(s.chars().nth(i));
            result += if d < Solution::char_to_int(s.chars().nth(i + 1)) {
                -d.unwrap()
            } else {
                d.unwrap()
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
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
