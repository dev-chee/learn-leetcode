//! Largest common sub-sequence

fn lcs_length(x: &[char], y: &[char]) -> (Vec<char>, Vec<i32>) {
    let m = x.len();
    let n = y.len();

    let mut b = vec![' '; (m + 1) * (n + 1)];
    let mut c = vec![0; (m + 1) * (n + 1)];

    for j in 1..=n {
        for i in 1..=m {
            if x[i - 1] == y[j - 1] {
                b[(m + 1) * j + i] = '↖';
                c[(m + 1) * j + i] = c[(m + 1) * (j - 1) + (i - 1)] + 1;
            } else {
                let tp = c[(m + 1) * (j - 1) + i];
                let lf = c[(m + 1) * j + (i - 1)];

                if tp >= lf {
                    b[(m + 1) * j + i] = '↑';
                    c[(m + 1) * j + i] = tp;
                } else {
                    b[(m + 1) * j + i] = '←';
                    c[(m + 1) * j + i] = lf;
                }
            }
        }
    }

    (b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = vec!['B', 'D', 'C', 'A', 'B', 'A'];
        let y = vec!['A', 'B', 'C', 'B', 'D', 'A', 'B'];
        // let lcs = vec!['B', 'C', 'B', 'A'];

        let (b, c) = lcs_length(&x, &y);

        print!("{:4}", ' ');
        print!("{:4}", 'j');
        for i in &x {
            print!("{:4}", i);
        }
        println!();

        let m = x.len() + 1;
        let n = y.len() + 1;
        for j in 0..n {
            if j == 0 {
                print!("{:3}", 'i');
            } else {
                print!("{:3}", y[j - 1]);
            }

            for i in 0..m {
                let x = m * j + i;
                print!("{}{}  ", b[x], c[x]);
            }

            println!();
        }
    }
}