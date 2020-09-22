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

impl ops::Div<i64> for ModInt {
    type Output = ModInt;
    fn div(self, _rhs: i64) -> ModInt {
        ModInt {
            val: self.val * inv(_rhs),
        }
    }
}

impl ModInt {
    pub fn pow(mut base: i64, mut exp: i64) -> ModInt {
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

    pub fn inv(base: i64) -> ModInt {
        pow(base, MOD - 2)
    }
}
