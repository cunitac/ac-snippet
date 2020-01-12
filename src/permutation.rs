#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn next_permutation<T: Ord>(v: &mut [T]) -> bool {
    let len = v.len();
    if len <= 1 {
        return false;
    }
    let mut i = len - 1;
    loop {
        let i1 = i;
        i -= 1;
        if v[i] < v[i1] {
            let mut i2 = len - 1;
            while v[i] >= v[i2] {
                i2 -= 1;
            }
            v.swap(i, i2);
            for j in 0..(len - i1) >> 1 {
                v.swap(i1 + j, len - j - 1);
            }
            return true;
        }
        if i == 0 {
            v.reverse();
            return false;
        }
    }
}

#[test]
fn test_next_permutation() {
    let mut v = [1, 2, 3];
    next_permutation(&mut v);
    assert_eq!(&v, &[1, 3, 2]);
    next_permutation(&mut v);
    assert_eq!(&v, &[2, 1, 3]);
    next_permutation(&mut v);
    assert_eq!(&v, &[2, 3, 1]);
    next_permutation(&mut v);
    next_permutation(&mut v);
    assert!(!next_permutation(&mut v));
}
