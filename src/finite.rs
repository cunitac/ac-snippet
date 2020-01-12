use cargo_snippet::snippet;

#[snippet("finite")]
const MOD: u64 = 1_000_000_007;

#[snippet("finite")]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Finite(u64);

#[snippet("finite")]
impl Finite {
    pub fn new(n: u64) -> Self {
        Finite(n % MOD)
    }
    pub fn pow(self, mut exp: u32) -> Self {
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
        Finite(acc)
    }
}

#[snippet("finite")]
impl std::fmt::Display for Finite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[snippet("finite")]
impl std::ops::Add for Finite {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Finite((self.0 + rhs.0) % MOD)
    }
}
#[snippet("finite")]
impl std::ops::Add<u64> for Finite {
    type Output = Self;
    fn add(self, rhs: u64) -> Self {
        Finite((self.0 + rhs) % MOD)
    }
}
#[snippet("finite")]
impl std::ops::AddAssign for Finite {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0) % MOD;
    }
}
#[snippet("finite")]
impl std::ops::AddAssign<u64> for Finite {
    fn add_assign(&mut self, rhs: u64) {
        self.0 = (self.0 + rhs) % MOD;
    }
}
#[snippet("finite")]
impl std::ops::Sub for Finite {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Finite((self.0 + MOD - rhs.0) % MOD)
    }
}
#[snippet("finite")]
impl std::ops::Sub<u64> for Finite {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self {
        Finite((self.0 + MOD - rhs) % MOD)
    }
}
#[snippet("finite")]
impl std::ops::SubAssign for Finite {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + MOD - rhs.0) % MOD;
    }
}
#[snippet("finite")]
impl std::ops::SubAssign<u64> for Finite {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 = (self.0 + MOD - rhs) % MOD;
    }
}
#[snippet("finite")]
impl std::ops::Mul for Finite {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Finite(self.0 * rhs.0 % MOD)
    }
}
#[snippet("finite")]
impl std::ops::Mul<u64> for Finite {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self {
        Finite(self.0 * rhs % MOD)
    }
}
#[snippet("finite")]
impl std::ops::MulAssign for Finite {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0 * rhs.0 % MOD;
    }
}
#[snippet("finite")]
impl std::ops::MulAssign<u64> for Finite {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 = self.0 * rhs % MOD;
    }
}
#[snippet("finite")]
impl std::ops::Div for Finite {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.pow((MOD - 2) as u32)
    }
}
#[snippet("finite")]
impl std::ops::Div<u64> for Finite {
    type Output = Self;
    fn div(self, rhs: u64) -> Self {
        self * Finite(rhs).pow((MOD - 2) as u32)
    }
}
#[snippet("finite")]
impl std::ops::DivAssign for Finite {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self * rhs.pow((MOD - 2) as u32);
    }
}
#[snippet("finite")]
impl std::ops::DivAssign<u64> for Finite {
    fn div_assign(&mut self, rhs: u64) {
        *self = *self * Finite(rhs).pow((MOD - 2) as u32);
    }
}

#[test]
fn test_finite() {
    let one = Finite(1);
    let mil = Finite(1_000_000);
    let max = Finite(MOD - 1);
    assert_eq!(one / mil * mil, one);
    assert_eq!(one / max * max, one);
    assert_eq!(mil * max / max, mil);
    assert_eq!(max * max, one)
}
