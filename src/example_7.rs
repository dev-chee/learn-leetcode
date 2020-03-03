//! 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
//!
//! 示例 1:
//!
//! 输入: 123
//! 输出: 321
//!
//! 示例 2:
//!
//! 输入: -123
//! 输出: -321
//!
//! 示例 3:
//!
//! 输入: 120
//! 输出: 21
//!
//! 注意:
//!
//! 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−231,  231 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode-cn.com/problems/reverse-integer
//! 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut rev = 0;

    while x != 0 {
        let pop = x % 10;
        x /= 10;

        if rev > i32::max_value() / 10 || (rev == i32::max_value() / 10 && pop > 7) {
            return 0;
        }

        if rev < i32::min_value() / 10 || (rev == i32::min_value() / 10 && pop < -8) {
            return 0;
        }

        rev = rev * 10 + pop;
    }

    return rev;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn test_4() {
        assert_eq!(reverse(i32::min_value()), 0);
    }
}