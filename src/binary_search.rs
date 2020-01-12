#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn bis_usize<P>(t: usize, f: usize, eval: P) -> usize
where
    P: Fn(usize) -> bool,
{
    if t > f {
        if t - f == 1 {
            return t;
        }
    } else if f - t == 1 {
        return t;
    }
    let m = (t + f) >> 1;
    if eval(m) {
        bis_usize(m, f, eval)
    } else {
        bis_usize(t, m, eval)
    }
}

#[snippet]
fn bis_isize<P>(t: isize, f: isize, eval: P) -> isize
where
    P: Fn(isize) -> bool,
{
    match t - f {
        1 | -1 => return t,
        _ => (),
    }
    let m = (t + f) >> 1;
    if eval(m) {
        bis_isize(m, f, eval)
    } else {
        bis_isize(t, m, eval)
    }
}

#[test]
fn test_bis_usize() {
    assert_eq!(bis_usize(100, 0, |x| x * x > 100), 11);
    assert_eq!(bis_usize(0, 100, |x| x * x < 100), 9);
}

#[snippet]
fn bis_f64<P>(t: f64, f: f64, err: f64, eval: P) -> f64
where
    P: Fn(f64) -> bool,
{
    if (t - f).abs() < err {
        return t;
    }
    let m = (t + f) / 2.;
    if eval(m) {
        bis_f64(m, f, err, eval)
    } else {
        bis_f64(t, m, err, eval)
    }
}

#[test]
fn test_bis_f64() {
    let bis = bis_f64(100., 0., 0.1, |x| x * x > 100.);
    assert!(10. < bis && bis < 10.1);
    let bis = bis_f64(0., 100., 0.1, |x| x * x < 100.);
    assert!(9.9 < bis && bis < 10.);
}
