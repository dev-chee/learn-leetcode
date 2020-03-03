pub fn insertion_sort<T>(values: &mut [T]) where T: PartialOrd
{
    if values.len() <= 1 {
        return;
    }

    for i in 1..values.len() {
        for j in (0..i).rev()
        {
            if values[j + 1] < values[j] {
                values.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v = [31, 41, 59, 26, 41, 58];
        insertion_sort(&mut v);
        assert_eq!([26, 31, 41, 41, 58, 59], v);
    }
}