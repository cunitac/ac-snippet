#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet("finite")]
trait IntoU64 {
    fn into_u64(self) -> u64;
}

#[snippet("finite")]
macro_rules! impl_into_u64 {
    ($($target:ty),*) => {$(
        impl IntoU64 for $target {
            fn into_u64(self) -> u64 {
                self as u64
            }
        }
    )*}
}

#[snippet("finite")]
impl_into_u64!(usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[snippet("finite")]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Fin(u64);

#[snippet("finite")]
const MOD: u64 = 1_000_000_007;

#[snippet("finite")]
impl Fin {
    fn from<T: IntoU64>(n: T) -> Fin {
        Fin(n.into_u64() % MOD)
    }
    fn pow(self, mut exp: u64) -> Fin {
        let mut base = self.0;
        let mut acc = 1;
        while exp > 1 {
            if (exp & 1) == 1 {
                acc = acc * base % MOD;
            }
            exp >>= 1;
            base = base * base % MOD;
        }
        if exp == 1 {
            acc = acc * base % MOD;
        }
        Fin(acc)
    }
    fn recip(self) -> Fin {
        self.pow(MOD - 2)
    }
}

#[snippet("finite")]
use std::fmt::{Display, Error, Formatter};

#[snippet("finite")]
impl Display for Fin {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

#[snippet("finite")]
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[snippet("finite")]
impl Neg for Fin {
    type Output = Fin;
    fn neg(self) -> Fin {
        Fin::from(MOD - self.0)
    }
}

#[snippet("finite")]
impl Add for Fin {
    type Output = Fin;
    fn add(self, other: Fin) -> Fin {
        Fin::from(self.0 + other.0)
    }
}

#[snippet("finite")]
impl Sub for Fin {
    type Output = Fin;
    fn sub(self, other: Fin) -> Fin {
        Fin::from(MOD + self.0 - other.0)
    }
}

#[snippet("finite")]
impl Mul for Fin {
    type Output = Fin;
    fn mul(self, other: Fin) -> Fin {
        Fin::from(self.0 * other.0)
    }
}

#[snippet("finite")]
impl Div for Fin {
    type Output = Fin;
    fn div(self, other: Fin) -> Fin {
        self * other.recip()
    }
}

#[snippet("finite")]
macro_rules! impl_fin_ops {
    ($($op:ident, $op_t:ident, $asn:ident, $asn_t:ident);*) => {$(
        impl<T: IntoU64> $op_t<T> for Fin {
            type Output = Fin;
            fn $op(self, other: T) -> Fin {
                self.$op(Fin::from(other))
            }
        }
        impl $asn_t for Fin {
            fn $asn(&mut self, rhs: Fin) {
                *self = (*self).$op(rhs);
            }
        }
        impl<T: IntoU64> $asn_t<T> for Fin {
            fn $asn(&mut self, rhs: T) {
                *self = (*self).$op(rhs);
            }
        }
    )*};
}

#[snippet("finite")]
impl_fin_ops!(add, Add, add_assign, AddAssign; sub, Sub, sub_assign, SubAssign; mul, Mul, mul_assign, MulAssign; div, Div, div_assign, DivAssign);

#[snippet("finite")]
use std::iter::{Product, Sum};

#[snippet("finite")]
impl Sum for Fin {
    fn sum<I: Iterator<Item = Fin>>(iter: I) -> Fin {
        iter.fold(Fin(0), Add::add)
    }
}

#[snippet("finite")]
impl<'a> Sum<&'a Fin> for Fin {
    fn sum<I: Iterator<Item = &'a Fin>>(iter: I) -> Fin {
        iter.fold(Fin(0), |a, &b| a.add(b))
    }
}

#[snippet("finite")]
impl Product for Fin {
    fn product<I: Iterator<Item = Fin>>(iter: I) -> Fin {
        iter.fold(Fin(0), Mul::mul)
    }
}

#[snippet("finite")]
impl<'a> Product<&'a Fin> for Fin {
    fn product<I: Iterator<Item = &'a Fin>>(iter: I) -> Fin {
        iter.fold(Fin(0), |a, &b| a.mul(b))
    }
}

#[test]
fn finite_test() {
    assert_eq!(Fin::from(MOD), Fin(0));
    assert_eq!(Fin::from(MOD as usize), Fin(0));
    assert_eq!(Fin(1000000).recip() * Fin(1000000), Fin(1));
}
