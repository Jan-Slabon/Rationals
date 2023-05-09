use cauchy::Scalar;
use num_traits::{One, Zero};
use std::fmt::Error;
use std::ops::{DivAssign,RemAssign, MulAssign, SubAssign, AddAssign, Rem, Sub};
use std::{iter::{Product, Sum},fmt::{UpperExp,LowerExp,Display},ops::{Neg}};
use num_traits::{cast::NumCast, ToPrimitive, FromPrimitive, Num};
use crate::Number;
use crate::Fraction;

// impl Scalar for Number{

// }
impl Num for Number{
    type FromStrRadixErr = Error;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match f32::from_str_radix(str, radix){
            Ok(x) => Ok(Number::Decimal(x)),
            Err(p) => Err(Error),
        }
        
    }
}
impl Zero for Number{
    fn zero() -> Self {
        Number::Fractional(Fraction { nominator: 0, denominator: 1})
    }
    fn is_zero(&self) -> bool {
        Number::zero() == *self
    }
}
impl One for Number{
    fn one() -> Self {
        Number::Fractional(Fraction { nominator: 1, denominator: 1})
    }
}

impl Sub for Number{
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl Rem for Number{
    type Output = Number;
    fn rem(self, rhs: Self) -> Self::Output {
      let x = self.to_f32().unwrap();
      let y = rhs.to_f32().unwrap();
      Number::Decimal(x % y)
    }
}
impl AddAssign for Number{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for Number{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + (-rhs);
    }
}
impl MulAssign for Number{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl DivAssign for crate::Number{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self/rhs;
    }
}
impl RemAssign for Number{
    fn rem_assign(&mut self, rhs: Self) {
        let x = self.to_f64().unwrap();
        let y = rhs.to_f64().unwrap();
        *self = Number::Decimal(x as f32 % y as f32);
    }
}
impl FromPrimitive for Number{
    fn from_u64(n: u64) -> Option<Self> {
        Some(Number::create_number(n as i32, 1))
    }
    fn from_f32(n: f32) -> Option<Self> {
        Some(Number::Decimal(n))
    }
    fn from_f64(n: f64) -> Option<Self> {
        Some(Number::Decimal(n as f32))
    }
    fn from_i32(n: i32) -> Option<Self> {
        Some(Number::Fractional(Fraction{nominator:n, denominator:1}))
    }
    fn from_i64(n: i64) -> Option<Self> {
        Some(Number::Fractional(Fraction{nominator:n as i32, denominator:1}))
    }
}
impl ToPrimitive for Number{
    fn to_u64(&self) -> Option<u64> {
        match self{
            Number::Fractional(Fraction{nominator, denominator}) => Some( *nominator as u64 / *denominator as u64),
            Number::Decimal(x) => Some(*x as u64),
        }
    }
    fn to_i64(&self) -> Option<i64> {
        match self{
            Number::Fractional(Fraction{nominator, denominator}) => Some( *nominator as i64 / *denominator as i64),
            Number::Decimal(x) => Some(*x as i64),
        }
    }
    fn to_f64(&self) -> Option<f64> {
        match self{
            Number::Fractional(Fraction{nominator, denominator}) => Some(*nominator as f64 / *denominator as f64),
            Number::Decimal(x) => Some(*x as f64),
        }
    }
    fn to_f32(&self) -> Option<f32> {
        match self{
            Number::Fractional(Fraction{nominator, denominator}) => Some(*nominator as f32 / *denominator as f32),
            Number::Decimal(x) => Some(*x),
        }
    }
}
impl NumCast for Number{
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(Number::Decimal(n.to_f32().unwrap()))
    }
}
impl Neg for Number{
    type Output = Number;
    fn neg(self) -> Self::Output {
        match self{
            Number::Fractional(Fraction{nominator, denominator}) => Number::Fractional(Fraction{nominator: -nominator, denominator: denominator}),
            Number::Decimal(x) => Number::Decimal(-x),
        }
    }
}
impl Display for Number{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Number::Fractional(frac) => Ok(print!("{}/{} ", frac.nominator, frac.denominator)),
            Number::Decimal(dec) => Ok(print!("{}", dec)),
        }
    }
}
impl LowerExp for Number{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = Number::to_decimal(self);
        LowerExp::fmt(&val, f)
    }
}
impl UpperExp for Number{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = Number::to_decimal(self);
        UpperExp::fmt(&val, f)
    }
}
impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Number::create_number(0, 1), |x,y| x+y)
    }
}
impl Product for Number{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Number::create_number(0,1), |x,y| x * y)
    }
}