#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        1 => 1,
        _ => gcd(b, a % b)
    }
}

#[snippet]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(25, 100), 25);
    assert_eq!(gcd(12, 30), 6);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(6, 8), 24);
    assert_eq!(lcm(1, 100), 100);
}

#[snippet]
fn primes(n: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    let mut ipm = vec![true; n + 1];
    let nsr = (n as f64).sqrt() as usize;
    for p in 2..nsr + 1 {
        if ipm[p] {
            let mut mpp = 2 * p;
            while mpp <= n {
                ipm[mpp] = false;
                mpp += p;
            }
        }
    }
    for p in 2..n + 1 {
        if ipm[p] {
            ret.push(p);
        }
    }
    ret
}

#[snippet]
#[snippet(include = "primes")]
fn factorize(n: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let mut n = n;
    let nsr = (n as f64).sqrt() as usize;
    for &p in &primes(nsr + 1) {
        let mut c = 0;
        while n % p == 0 {
            c += 1;
            n /= p;
        }
        if c > 0 {
            ret.push((p, c));
        }
    }
    if n > 1 {
        ret.push((n, 1));
    }
    ret
}

#[test]
fn test_primes() {
    let prs = primes(1_000_000);
    assert_eq!(&prs[0..5], &[2, 3, 5, 7, 11]);
    assert_eq!(prs[9591], 99991);
}

#[test]
fn test_factorize() {
    let ftr = factorize(174636000);
    assert_eq!(&ftr, &[(2, 5), (3, 4), (5, 3), (7, 2), (11, 1)]);
}
