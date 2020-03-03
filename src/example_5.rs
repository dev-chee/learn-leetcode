//! 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
//!
//! 示例 1：
//!
//! 输入: "babad"
//! 输出: "bab"
//! 注意: "aba" 也是一个有效答案。
//!
//! 示例 2：
//!
//! 输入: "cbbd"
//! 输出: "bb"
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode-cn.com/problems/longest-palindromic-substring
//! 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use std::cmp;
use std::iter::FromIterator;

pub fn longest_palindrome(s: String) -> String {
    let chars = s.chars().collect::<Vec<_>>();

    if chars.len() == 0 || chars.len() == 1 {
        return s;
    }

    let (mut start, mut end) = (0_usize, 0_usize);

    for i in 0..chars.len() {
        let len1 = expand_around(&chars, (i, i));
        let len2 = expand_around(&chars, (i, i + 1));
        let len = cmp::max(len1, len2);

        if len > end - start {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }


    return String::from_iter(&chars[start..end + 1]);
}

fn expand_around(chars: &[char], range: (usize, usize)) -> usize {
    let (mut left, mut right) = range;

    while right < chars.len() {
        if chars[left] != chars[right] {
            return right - left - 1;
        }

        if left == 0 || right == chars.len() - 1 {
            return right - left + 1;
        }

        left -= 1;
        right += 1;
    }

    return right - left - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "babad".to_string();
        assert_eq!(longest_palindrome(s), "aba".to_string());
    }

    #[test]
    fn test_2() {
        let s = "cbbd".to_string();
        assert_eq!(longest_palindrome(s), "bb".to_string());
    }

    #[test]
    fn test_3() {
        let s = "".to_string();
        assert_eq!(longest_palindrome(s), "".to_string());
    }

    #[test]
    fn test_4() {
        let s = "b".to_string();
        assert_eq!(longest_palindrome(s), "b".to_string());
    }

    #[test]
    fn test_5() {
        let s = "bb".to_string();
        assert_eq!(longest_palindrome(s), "bb".to_string());
    }

    #[test]
    fn test_6() {
        let s = "bbbbb".to_string();
        assert_eq!(longest_palindrome(s), "bbbbb".to_string());
    }

    #[test]
    fn test_7() {
        let s = "abccbacdaaaaa".to_string();
        assert_eq!(longest_palindrome(s), "abccba".to_string());
    }
}