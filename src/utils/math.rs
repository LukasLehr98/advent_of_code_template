pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 && b == 0 {
        return 0;
    }
    (a * b).abs() / gcd(a, b)
}

pub fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

// Chinese Remainder Theorem
pub fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    if residues.len() != modulii.len() { return None; }
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inverse(p, modulus)? * p
    }
    Some(sum % prod)
}

fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let mut mn = (m, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    if mn.0 > 1 { return None; }
    Some((xy.0 + m) % m)
} 