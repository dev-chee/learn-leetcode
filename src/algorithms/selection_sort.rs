fn find_min<T>(values: &[T]) -> Option<usize> where T: PartialOrd {
    if values.is_empty() {
        return None;
    }

    let mut idx = 0;
    let mut min = &values[idx];

    for i in 1..values.len() {
        let val = &values[i];

        if val < min {
            idx = i;
            min = val;
        }
    }

    Some(idx)
}

pub fn selection_sort<T>(values: &mut [T]) -> bool where T: PartialOrd
{
    if values.is_empty() {
        return false;
    }

    for i in 0..values.len() {
        let idx = find_min(&values[i..]);
        if idx.is_none() {
            return false;
        }

        let idx = idx.unwrap();
        if idx != 0 {
            values.swap(i, i + idx);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v = [2, 3, 10, 7, 15, 22, 18, 23, 76, 100, 38, 92];
        selection_sort(&mut v);
        assert_eq!([2, 3, 7, 10, 15, 18, 22, 23, 38, 76, 92, 100], v);
    }
}