fn find_max_crossing_sub_array(values: &[i32], low: usize, mid: usize, high: usize)
                               -> (usize, usize, i32)
{
    let mut left_sum = std::i32::MIN;
    let mut left_idx = mid;
    let mut sum = 0;

    for i in (low..=mid).rev() {
        sum += values[i];

        if sum > left_sum {
            left_sum = sum;
            left_idx = i;
        }
    }

    let mut right_sum = std::i32::MIN;
    let mut right_idx = mid + 1;
    let mut sum = 0;

    for i in mid + 1..=high {
        sum += values[i];

        if sum > right_sum {
            right_sum = sum;
            right_idx = i;
        }
    }

    (left_idx, right_idx, left_sum + right_sum)
}

pub fn find_max_sub_array(values: &[i32], low: usize, high: usize)
                          -> (usize, usize, i32)
{
    if low == high {
        return (low, high, values[low]);
    }

    let mid = low + (high - low) / 2;

    let (left_low, left_high, left_sum) = find_max_sub_array(values, low, mid);
    let (right_low, right_high, right_sum) = find_max_sub_array(values, mid + 1, high);
    let (cross_low, cross_high, cross_sum) = find_max_crossing_sub_array(values, low, mid, high);

    if left_sum >= right_sum && left_sum >= cross_sum {
        (left_low, left_high, left_sum)
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        (right_low, right_high, right_sum)
    } else {
        (cross_low, cross_high, cross_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = [-1, 10, -2, 14, 17];
        assert_eq!(find_max_sub_array(&v, 0, v.len() - 1), (1, 4, 39));
    }

    #[test]
    fn test_neg() {
        let v = [-1, -10, -2, -14, -17];
        assert_eq!(find_max_sub_array(&v, 0, v.len() - 1), (0, 0, -1));
    }
}