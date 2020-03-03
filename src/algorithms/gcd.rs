use std::mem;

pub fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;

    if x == 0 {
        return y;
    }

    if y == 0 {
        return x;
    }

    if y > x {
        mem::swap(&mut x, &mut y);
    }

    let r = x % y;
    gcd(y, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(gcd(1680, 640), 80);
        assert_eq!(gcd(640, 1680), 80);
    }
}