use proconio::*;
use std::ops;

const MOD: i64 = 1_000_000_007;

pub struct ModInt {
    val: i64,
}

impl ModInt {
    pub fn new(val: i64) -> ModInt {
        ModInt { val }
    }
}

impl ops::Add<i64> for ModInt {
    type Output = ModInt;
    fn add(self, _rhs: i64) -> ModInt {
        ModInt {
            val: (self.val + _rhs) % MOD,
        }
    }
}

impl ops::Add<ModInt> for ModInt {
    type Output = ModInt;
    fn add(self, _rhs: ModInt) -> ModInt {
        ModInt {
            val: (self.val + _rhs.val) % MOD,
        }
    }
}

impl ops::Sub<i64> for ModInt {
    type Output = ModInt;
    fn sub(self, _rhs: i64) -> ModInt {
        ModInt {
            val: (self.val - _rhs + MOD) % MOD,
        }
    }
}

impl ops::Sub<ModInt> for ModInt {
    type Output = ModInt;
    fn sub(self, _rhs: ModInt) -> ModInt {
        ModInt {
            val: (self.val - _rhs.val + MOD) % MOD,
        }
    }
}

impl ops::Mul<i64> for ModInt {
    type Output = ModInt;
    fn mul(self, _rhs: i64) -> ModInt {
        ModInt {
            val: (self.val * _rhs) % MOD,
        }
    }
}

impl ops::Mul<ModInt> for ModInt {
    type Output = ModInt;
    fn mul(self, _rhs: ModInt) -> ModInt {
        ModInt {
            val: (self.val * _rhs.val) % MOD,
        }
    }
}

impl ModInt {
    pub fn pow(self, mut base: i64, mut exp: i64) -> ModInt {
        let mut res = ModInt::new(1i64);
        while exp > 0 {
            if (exp & 1) > 0 {
                res = res * base;
            }
            exp >>= 1;
            base = base * base % MOD;
        }
        res
    }

    pub fn inv(self, base: i64) -> ModInt {
        self.pow(base, MOD - 2)
    }
}

fn main() {
    input! {
        n: i64,
    }
    let ten = ModInt::new(1).pow(10, n);
    let nine = ModInt::new(1).pow(9, n);
    let eight = ModInt::new(1).pow(8, n);

    println!("{}", (ten - nine * 2 + eight).val);
}
