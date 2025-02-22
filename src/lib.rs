use rand::{rngs::ThreadRng, Rng};

fn calculate_y(x: i64, pol: &[i64]) -> i64 {
    pol.iter()
        .enumerate()
        .fold(0, |acc, (i, &p)| acc + p * x.pow(i as u32))
}

fn generate_unique(rgn: &mut ThreadRng, v: &[i64]) -> i64 {
    let mut r: i64 = rgn.random();
    r %= 997;

    match v.contains(&r) {
        true => generate_unique(rgn, v),
        false => r,
    }
}

fn generate_unique_2(rgn: &mut ThreadRng, v: &[(i64, i64)]) -> (i64, i64) {
    let mut r1: i64 = rgn.random();
    let mut r2: i64 = rgn.random();

    r1 %= 997;
    r2 %= 997;

    match v.contains(&(r1, r2)) {
        true => generate_unique_2(rgn, v),
        false => (r1, r2),
    }
}

fn generate_pol(key: i64, k: u64, rgn: &mut ThreadRng) -> Vec<i64> {
    let mut pol: Vec<i64> = vec![key];

    for _i in 1..k {
        let r: i64 = generate_unique(rgn, &pol);
        pol.push(r);
    }

    pol
}

pub fn create_secret_shares(key: i64, k: u64, n: u64) -> Vec<(i64, i64)> {
    let mut rgn = rand::rng();

    let pol = generate_pol(key, k, &mut rgn);
    let mut shares: Vec<(i64, i64)> = Vec::new();

    for _i in 0..n {
        let (r1, r2) = generate_unique_2(&mut rgn, &shares);
        shares.push((calculate_y(r1, &pol), calculate_y(r2, &pol)));
    }

    shares
}

pub fn recover_secret(shares: &[(i64, i64)]) -> i64 {}

#[test]
fn test_create_recover() {
    let mut rgn = rand::rng();

    let key: i64 = rgn.random();
    let k = 2;
    let n = 5;

    let shares = create_secret_shares(key, k, n);

    let recovered_key = recover_secret(&shares);

    assert_eq!(key, recovered_key, "{key} compared to {recovered_key}");
}
