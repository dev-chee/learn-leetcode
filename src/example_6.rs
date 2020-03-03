//! 将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
//!
//! 比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：
//!
//! L   C   I   R
//! E T O E S I I G
//! E   D   H   N
//!
//! 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。
//!
//! 请你实现这个将字符串进行指定行数变换的函数：
//!
//! string convert(string s, int numRows);
//!
//! 示例 1:
//!
//! 输入: s = "LEETCODEISHIRING", numRows = 3
//! 输出: "LCIRETOESIIGEDHN"
//!
//! 示例 2:
//!
//! 输入: s = "LEETCODEISHIRING", numRows = 4
//! 输出: "LDREOEIIECIHNTSG"
//! 解释:
//!
//! L     D     R
//! E   O E   I I
//! E C   I H   N
//! T     S     G
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode-cn.com/problems/zigzag-conversion
//! 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use std::cmp;
use std::iter::FromIterator;

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
        return s;
    }

    let num_rows = num_rows as usize;
    let chars = s.chars().collect::<Vec<_>>();

    if num_rows >= chars.len() {
        return s;
    }

    let mut total_rows = Vec::with_capacity(num_rows);
    let mut cur_row = 0;
    let mut downward = false;

    for ch in chars {
        if total_rows.len() < (cur_row + 1) {
            total_rows.push(Vec::new());
        }

        total_rows[cur_row].push(ch);

        if cur_row == 0 || cur_row == (num_rows - 1) {
            downward = !downward;
        }

        if downward {
            cur_row += 1;
        } else {
            cur_row -= 1;
        }
    }

    let mut result = String::new();

    for row in total_rows {
        result.push_str(&String::from_iter(row.into_iter()));
    }

    return result;
}

pub fn convert_2(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
        return s;
    }

    let num_rows = num_rows as usize;
    let chars = s.chars().collect::<Vec<_>>();
    let cycle = 2 * num_rows - 2;

    if num_rows >= chars.len() {
        return s;
    }

    let mut result = Vec::new();

    for i in 0..num_rows {
        let mut j = 0;
        while j + i < chars.len() {
            result.push(chars[j + i]);

            if i != 0 && i != (num_rows - 1) && (j + cycle - i) < chars.len() {
                result.push(chars[j + cycle - i]);
            }

            j += cycle;
        }
    }

    return String::from_iter(result.into_iter());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "LEETCODEISHIRING".to_string();
        let num_rows = 3;

        assert_eq!(convert(s, num_rows), "LCIRETOESIIGEDHN".to_string());
    }

    #[test]
    fn test_2() {
        let s = "LEETCODEISHIRING".to_string();
        let num_rows = 4;

        assert_eq!(convert(s, num_rows), "LDREOEIIECIHNTSG".to_string());
    }

    #[test]
    fn test_3() {
        let s = "LEETCODEISHIRING".to_string();
        let num_rows = 3;

        assert_eq!(convert_2(s, num_rows), "LCIRETOESIIGEDHN".to_string());
    }

    #[test]
    fn test_4() {
        let s = "LEETCODEISHIRING".to_string();
        let num_rows = 4;

        assert_eq!(convert_2(s, num_rows), "LDREOEIIECIHNTSG".to_string());
    }
}