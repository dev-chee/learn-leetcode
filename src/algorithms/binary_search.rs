pub fn binary_search<T>(values: &[T], target: &T) -> Option<usize>
    where T: PartialEq + PartialOrd
{
    if values.is_empty() {
        return None;
    }

    let mut beg = 0;
    let mut end = values.len() - 1;

    while beg <= end {
        let mid = beg + (end - beg) / 2;
        let val = &values[mid];

        if val == target {
            return Some(mid);
        } else if val > target {
            end = mid - 1;
        } else {
            beg = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = [1, 4, 10, 24, 50, 100, 280, 890];

        assert_eq!(binary_search(&v, &280), Some(6));
        assert_eq!(binary_search(&v, &24), Some(3));
    }
}