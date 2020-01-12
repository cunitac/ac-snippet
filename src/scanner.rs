#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet("Scanner")]
struct Scanner<'a>(std::str::SplitWhitespace<'a>);

#[snippet("Scanner")]
impl<'a> Scanner<'a> {
    fn from_str(src: &str) -> Scanner {
        Scanner(src.split_whitespace())
    }
    fn get(&mut self) -> &str {
        self.0.next().unwrap()
    }
    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.get().parse().ok().unwrap()
    }
}

#[test]
fn test_scanner() {
    let src = r"14
    atcoder
    rust
    1 1 2 3 4 5
    ";
    let mut sc = Scanner::from_str(src);
    assert_eq!(sc.read::<usize>(), 14usize);
    assert_eq!(sc.read::<String>(), "atcoder");
    assert_eq!(sc.get(), "rust");
    assert_eq!((0..6).map(|_| sc.read::<u32>()).collect::<Vec<_>>(), vec![1u32, 1, 2, 3, 4, 5]);
}
