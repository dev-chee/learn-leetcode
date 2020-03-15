use std::cmp::max;

/// 分治：切割长度为`n`的钢条，使得销售价格最优，参数p为对应n段的价格列表
pub fn cut_rod(p: &[i32], n: usize) -> i32 {
    assert!(n < p.len());

    if n == 0 {
        return p[0];
    }

    let mut q = -1;

    for i in 1..=n {
        q = max(q, p[i] + cut_rod(p, n - i));
    }

    q
}

/// 动态规划：带备忘录避免无意义的递归计算相同的子问题
pub fn memorized_cut_rod(p: &[i32], n: usize) -> i32 {
    assert!(n < p.len());

    if n == 0 {
        return p[0];
    }

    let mut r = vec![-1; n + 1];
    r[0] = p[0];

    return memorized_cut_rod_aux(p, n, &mut r);
}

fn memorized_cut_rod_aux(p: &[i32], n: usize, r: &mut [i32]) -> i32 {
    if r[n] >= 0 {
        return r[n];
    }

    let mut q = -1;

    for i in 1..=n {
        q = max(q, p[i] + memorized_cut_rod_aux(p, n - i, r));
    }

    r[n] = q;
    q
}

/// 动态规划，自底向上
pub fn bottom_up_cut_rod(p: &[i32], n: usize) -> i32 {
    assert!(n < p.len());

    if n == 0 {
        return p[0];
    }

    let mut r = vec![-1; n + 1];
    r[0] = p[0];

    for j in 1..=n {
        let mut q = -1;

        for i in 1..=j {
            q = max(q, p[i] + r[j - i]);
        }

        r[j] = q;
    }

    r[n]
}


// 输入信息
// idx:     1    2    3    4    5    6    7    8    9   10
// pri:     1    5    8    9   10   17   17   20   24   30
// seg:     1    2    2    2    2    2    2    2    2    2
// rod:     1    5   10   15   20   25   30   35   40   45
pub fn extended_bottom_up_cut_rod(p: &[i32], n: usize) -> (Vec<i32>, Vec<i32>) {
    assert!(n < p.len());

    if n == 0 {
        return (vec![p[0]], vec![p[0]]);
    }

    let mut r = vec![-1; n + 1];
    r[0] = p[0];

    let mut s = vec![-1; n + 1];
    s[0] = 0;

    for j in 1..=n {
        let mut q = -1;

        for i in 1..=j {
            let x = p[i] + r[j - i];

            if q < x
            {
                q = x;
                s[j] = i as i32;
            }
        }

        r[j] = q;
    }

    (r, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = [0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        assert_eq!(cut_rod(&p, 4), 10);
        assert_eq!(memorized_cut_rod(&p, 4), 10);
        assert_eq!(bottom_up_cut_rod(&p, 4), 10);

        let (r, s) = extended_bottom_up_cut_rod(&p, p.len() - 1);

        let mut idx = String::from("idx: ");
        let mut pri = String::from("pri: ");
        let mut seg = String::from("seg: ");
        let mut rod = String::from("rod: ");

        for i in 0..p.len() {
            idx += &format!("{:5}", i);
            pri += &format!("{:5}", p[i]);
            seg += &format!("{:5}", s[i]);
            rod += &format!("{:5}", r[i]);
        }

        println!("{}", idx);
        println!("{}", pri);
        println!("{}", seg);
        println!("{}", rod);
    }
}