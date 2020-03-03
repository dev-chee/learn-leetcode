//! 给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。
//!
//! 请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
//!
//! 你可以假设 nums1 和 nums2 不会同时为空。
//!
//! 示例 1:
//!
//! nums1 = [1, 3]
//! nums2 = [2]
//!
//! 则中位数是 2.0
//! 示例 2:
//!
//! nums1 = [1, 2]
//! nums2 = [3, 4]
//!
//! 则中位数是 (2 + 3)/2 = 2.5
//!
//! 来源：力扣（LeetCode）
//! 链接：https://leetcode-cn.com/problems/median-of-two-sorted-arrays
//! 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use std::cmp;

/// 中位数: 将一个集合划分为两个长度相等的子集，其中一个子集中的元素总是大于另一个子集中的元素。
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut result = 0.0;
    let (nums1, nums2) = swap_if_greater(nums1, nums2);
    let (mut min_idx, mut max_idx) = (0_usize, nums1.len());
    let mid_idx = (nums1.len() + nums2.len() + 1) / 2;

    while min_idx <= max_idx {
        let i = (min_idx + max_idx) / 2;
        let j = mid_idx - i;

        if i < max_idx && nums2[j - 1] > nums1[i] { // i is smaller
            min_idx = i + 1;
        } else if i > min_idx && nums1[i - 1] > nums2[j] { // i is bigger
            max_idx = i - 1;
        } else { // i is fine
            let max_left = if i == 0 {
                nums2[j - 1]
            } else if j == 0 {
                nums1[i - 1]
            } else {
                cmp::max(nums1[i - 1], nums2[j - 1])
            };

            if (nums1.len() + nums2.len()) % 2 == 1 {
                result = max_left as f64;
                break;
            }

            let min_right = if i == nums1.len() {
                nums2[j]
            } else if j == nums2.len() {
                nums1[i]
            } else {
                cmp::min(nums1[i], nums2[j])
            };

            result = (max_left + min_right) as f64 / 2.0;
            break;
        }
    }

    result
}

fn swap_if_greater(x: Vec<i32>, y: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    if x.len() > y.len() {
        (y, x)
    } else {
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let x = vec![1, 3];
        let y = vec![2];
        assert_eq!(find_median_sorted_arrays(x, y), 2.0);
    }

    #[test]
    fn test_2() {
        let x = vec![1, 2];
        let y = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(x, y), 2.5);
    }

    #[test]
    fn test_3() {
        let x = vec![1, 3, 8, 12];
        let y = vec![2, 10, 12];

        assert_eq!(find_median_sorted_arrays(x, y), 8.0);
    }

    #[test]
    fn test_4() {
        let x = vec![1, 3, 8, 12];
        let y = vec![2, 3, 10, 12];

        assert_eq!(find_median_sorted_arrays(x, y), 5.5);
    }
}