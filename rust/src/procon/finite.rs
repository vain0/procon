use std;
use std::fmt::{Debug, Display, Formatter};
use std::ops::*;

const P: i64 = 1_000_000_007;

/// Calculates `x^n`. O(log n) time.
/// By Fermat's little theorem, `x^(-1) = pow(x, P - 2)`.
pub fn pow(x: i64, n: i64) -> i64 {
    let (mut x, mut y, mut n) = (x % P, 1_i64, n);
    while n > 0 {
        if n % 2 != 0 {
            y = (y * x) % P;
            n -= 1;
        }

        x = (x * x) % P;
        n /= 2;
    }
    y
}

/// Calculates `1/a` for each `a` in `1..n`.
/// Use `P = floor(P / k) * k + P % k` for proof.
pub fn inv_dp(n: usize) -> Vec<i64> {
    let mut dp = vec![0; n];
    if n >= 2 {
        dp[1] = 1;
        for i in 2..n {
            let mut z = P - dp[(P % i as i64) as usize];
            z %= P;
            z *= P / i as i64;
            z %= P;
            dp[i] = z;
        }
    }
    dp
}

/// Represents an element of finite field.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
struct Finite(i64);

impl Finite {
    fn pow(self, e: i64) -> Self {
        pow(self.0, e).into()
    }
}

impl Debug for Finite {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!("{:?}", self.0))
    }
}

impl Display for Finite {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!("{}", self.0))
    }
}

impl From<i64> for Finite {
    fn from(value: i64) -> Self {
        Finite((value % P + P) % P)
    }
}

// Generate binary operation traits.
macro_rules! impl_binary_op_for_finite {
    ($op_trait:ident, $op:ident, $assign_trait:ident, $assign:ident $(, $f:ident)*) => {
        $(impl $op_trait<Finite> for Finite {
            type Output = Self;

            fn $op(self, other: Self) -> Self {
                Finite::from((self.0).$f(other.0))
            }
        })*

        impl $op_trait<i64> for Finite {
            type Output = Self;

            fn $op(self, other: i64) -> Self {
                self.$op(Finite::from(other))
            }
        }

        impl $assign_trait<Finite> for Finite {
            fn $assign(&mut self, other: Self) {
                *self = self.$op(other)
            }
        }

        impl $assign_trait<i64> for Finite {
            fn $assign(&mut self, other: i64) {
                *self = self.$op(other)
            }
        }
    };
}

impl_binary_op_for_finite! {Add, add, AddAssign, add_assign, add}
impl_binary_op_for_finite! {Sub, sub, SubAssign, sub_assign, sub}
impl_binary_op_for_finite! {Mul, mul, MulAssign, mul_assign, mul}
impl_binary_op_for_finite! {Div, div, DivAssign, div_assign}

impl Div<Finite> for Finite {
    type Output = Finite;

    fn div(self, other: Self) -> Self {
        self * other.pow(P - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::{inv_dp, pow, Finite, P};
    use std;

    #[test]
    fn test_pow_edges() {
        assert_eq!(pow(0, 0), 1);
        assert_eq!(pow(2, 0), 1);
        assert_eq!(pow(3, 1), 3);
        assert_eq!(pow(5, 6), (5 * 5 * 5) * (5 * 5 * 5));
        assert_eq!(pow(std::i64::MAX % P, std::i64::MAX), 856225998);
    }

    #[test]
    fn test_pow_small() {
        for x in 0..100_i64 {
            for n in 0..100_i64 {
                let actual = pow(x, n);
                let expected = (0..n).fold(1_i64, |acc, _| (acc * x) % P);
                assert_eq!(actual, expected, "{}^{}", x, n);
            }
        }
    }

    #[test]
    fn test_inv_dp() {
        let n = 10000;
        let dp = inv_dp(n);
        for i in 1..n {
            let mut z = dp[i] * i as i64;
            z %= P;
            z += P;
            z %= P;
            assert_eq!(z, 1);
        }
    }

    #[test]
    fn test_finite() {
        let x = Finite::from(P + 2);

        // `from` should normalize the value.
        assert_eq!(x.0, 2);

        // Operations.
        assert_eq!(x + 7, (2 + 7).into());
        assert_eq!(x - 7, (P + (2 - 7)).into());
        assert_eq!(x * 3, (2 * 3).into());
        assert_eq!((x / 11) * 11, 2.into());

        assert_eq!(x + Finite::from(7), (2 + 7).into());
        assert_eq!(x - Finite::from(7), (P + (2 - 7)).into());
        assert_eq!(x * Finite::from(3), (2 * 3).into());
        assert_eq!((x / Finite::from(11)) * 11, 2.into());

        let mut x = x;
        x += 7;
        assert_eq!(x, 9.into());
        x -= 5;
        assert_eq!(x, 4.into());
        x *= 6;
        assert_eq!(x, 24.into());
        x /= 3;
        assert_eq!(x, 8.into());
    }

    #[test]
    fn test_finite_fmt() {
        assert_eq!(format!("{:?}", Finite::from(2)), "2");
        assert_eq!(format!("{}", Finite::from(2)), "2");
    }
}
