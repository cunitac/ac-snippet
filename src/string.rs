#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
/*
/// a[i] = s[0..i] の最長強境界
fn longest_strong_borders<T: PartialEq>(s: &[T]) -> Vec<Option<usize>> {
    let mut a = vec![None; s.len()+1];
    for i in 1..s.len()+1 {
        
    }
}*/

#[test]
fn test_longest_borders() {}

#[snippet]
/// a[2*i]   = iを中心とした最長の回文の長さ
/// a[2*i+1] = i+1を中心とした最長の回文の長さ
/// a[l+r] >= r-l+1 = s[l..r] は回文
fn longest_palindromes<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let mut a = vec![0; 2 * s.len() - 1];
    let mut i = 0;
    let mut j = 0;
    let get = |i| if i & 1 == 1 { None } else { Some(&s[i >> 1]) };
    while i < a.len() {
        while i >= j && i + j < a.len() && get(i - j) == get(i + j) {
            j += 1;
        }
        a[i] = j;
        let mut k = 1;
        while i >= k && i + k < a.len() && k + a[i - k] < j {
            a[i + k] = a[i - k];
            k += 1
        }
        i += k;
        j -= k;
    }
    a
}

#[snippet]
fn longest_common_prefixes<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let mut a = vec![0; s.len()];
    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        while i + j < s.len() && s[j] == s[i + j] {
            j += 1;
        }
        a[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < s.len() && k + a[k] < j {
            a[i + k] = a[k];
            k += 1
        }
        i += k;
        j -= k;
    }
    a
}
