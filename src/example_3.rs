//! 给定一个字符串，请你找出其中不含有重复字符的 *最长子串* 的长度。
//!
//! 示例 1:
//!
//! 输入: "abcabcbb"
//! 输出: 3
//! 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
//!
//! 示例 2:
//!
//! 输入: "bbbbb"
//! 输出: 1
//! 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
//!
//! 示例 3:
//!
//! 输入: "pwwkew"
//! 输出: 3
//! 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//!      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode-cn.com/problems/longest-substring-without-repeating-characters
//! 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::iter::Take;

pub fn length_of_longest_substring_1(s: String) -> i32 {
    let mut result = 0_usize;
    let chars = s.chars().collect::<Vec<_>>();

    // [0, 1, 2, 3, 4, 5]
    for i in 0..chars.len() {
        for j in (i + 1)..(chars.len() + 1) {
            if all_unique(&chars[i..j]) {
                result = cmp::max(result, j - i);
            }
        }
    }

    result as i32
}

fn all_unique(string: &[char]) -> bool {
    let mut cache = HashSet::<char>::new();
    let mut result = true;

    for ch in string {
        if cache.contains(ch) {
            result = false;
            break;
        }

        cache.insert(*ch);
    }

    result
}

pub fn length_of_longest_substring_2(s: String) -> i32 {
    let mut result = 0;
    let mut cache = HashSet::<char>::new();
    let chars = s.chars().collect::<Vec<_>>();
    let mut l = 0;
    let mut r = 0;

    while l < chars.len() && r < chars.len() {
        if !(cache.contains(&chars[r])) {
            cache.insert(chars[r]);
            r += 1;

            result = cmp::max(result, r - l);
        } else {
            cache.remove(&chars[l]);
            l += 1;
        }
    }

    result as i32
}

pub fn length_of_longest_substring_3(s: String) -> i32 {
    let mut result = 0;
    let chars = s.chars().collect::<Vec<_>>();
    let mut index_map = HashMap::<char, usize>::new();
    let mut l = 0_usize;

    for r in 0..chars.len() {
        if index_map.contains_key(&chars[r]) {
            l = cmp::max(*index_map.get(&chars[r]).unwrap() + 1, l)
        }

        result = cmp::max(result, r - l + 1);

        index_map.insert(chars[r], r);
    }


    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let x = "abcabcbb".to_string();
        assert_eq!(length_of_longest_substring_1(x), 3);
    }

    #[test]
    fn test_2() {
        let x = "abcabcbb".to_string();
        assert_eq!(length_of_longest_substring_2(x), 3);
    }

    #[test]
    fn test_3() {
        let x = "abcabcbb".to_string();
        assert_eq!(length_of_longest_substring_3(x), 3);
    }


    #[test]
    fn test_4() {
        let x = "bbbbb".to_string();
        assert_eq!(length_of_longest_substring_1(x), 1);
    }

    #[test]
    fn test_5() {
        let x = "bbbbb".to_string();
        assert_eq!(length_of_longest_substring_2(x), 1);
    }

    #[test]
    fn test_6() {
        let x = "bbbbb".to_string();
        assert_eq!(length_of_longest_substring_3(x), 1);
    }

    #[test]
    fn test_7() {
        let x = "pwwkew".to_string();
        assert_eq!(length_of_longest_substring_1(x), 3);
    }

    #[test]
    fn test_8() {
        let x = "pwwkew".to_string();
        assert_eq!(length_of_longest_substring_2(x), 3);
    }

    #[test]
    fn test_9() {
        let x = "pwwkew".to_string();
        assert_eq!(length_of_longest_substring_3(x), 3);
    }

    #[test]
    fn test_10() {
        let x = "pwwkexy".to_string();
        assert_eq!(length_of_longest_substring_1(x), 5);
    }

    #[test]
    fn test_11() {
        let x = "pwwkexy".to_string();
        assert_eq!(length_of_longest_substring_2(x), 5);
    }

    #[test]
    fn test_12() {
        let x = "pwwkexy".to_string();
        assert_eq!(length_of_longest_substring_3(x), 5);
    }
}