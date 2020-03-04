use rand::Rng;

pub fn quick_sort(values: &mut [i32]) {
    if values.len() <= 1 {
        return;
    }

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0, values.len());
    values.swap(0, idx);

    let mut p_idx = 0;
    let p = values[p_idx];

    for i in 1..values.len() {
        let c = values[i];
        if c <= p {
            rshift(values, i);
            p_idx += 1;
        }
    }

    quick_sort(&mut values[0..p_idx]);
    quick_sort(&mut values[p_idx + 1..]);
}

fn rshift(values: &mut [i32], idx: usize) {
    let x = values[idx];
    for i in (0..idx).rev() {
        values[i + 1] = values[i];
    }

    values[0] = x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v = [2, 3, 10, 7, 15, 22, 77, 11, 32, 41, 18, 23, 76, 100, 38, 92];
        quick_sort(&mut v);
        assert_eq!(v, [2, 3, 7, 10, 11, 15, 18, 22, 23, 32, 38, 41, 76, 77, 92, 100]);
    }
}