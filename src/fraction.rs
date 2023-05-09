use crate::euclid;
use serde::{Deserialize, Serialize};
#[derive(Debug,Copy,Clone,Deserialize,Serialize)]
pub struct Fraction{
    pub nominator : i32,
    pub denominator : i32
}

impl Fraction{
    pub fn add(&self, rhs : &Fraction) -> Fraction {
        let gcd = euclid::gcd(self.denominator, rhs.denominator);
        let left_nom : i32 = self.nominator * (rhs.denominator / gcd);
        let right_nom : i32 = rhs.nominator * (self.denominator / gcd);
        let denom : i32 =  (self.denominator * rhs.denominator) / gcd;
        let gcd_nom_denom = euclid :: gcd(left_nom + right_nom, denom);
        Fraction{nominator : (left_nom + right_nom) / gcd_nom_denom, denominator : denom / gcd_nom_denom}
    }
    pub fn mult(&self, rhs : &Fraction) -> Fraction {
        let new_nom = self.nominator * rhs.nominator;
        let new_denom = self.denominator * rhs.denominator;
        let gcd_nom_denom = euclid :: gcd(new_nom, new_denom);
        Fraction { nominator: new_nom / gcd_nom_denom, denominator: new_denom / gcd_nom_denom }
    }
    pub fn div(&self, rhs : &Fraction) -> Fraction {
        self.mult(&rhs.inverse())
    }
    pub fn inverse(&self) -> Fraction {
        Fraction { nominator: self.denominator, denominator: self.nominator}
    }
    pub fn pow(&self, x : u32) -> Fraction {
        let nom = i32 :: pow(self.nominator, x);
        let denom = i32 :: pow(self.denominator, x);
        let gcd_nom_denom = euclid::gcd(nom, denom);
        Fraction { nominator: nom / gcd_nom_denom, denominator: denom / gcd_nom_denom }
    }
}

impl PartialEq for Fraction{
    fn eq(&self, other: &Fraction) -> bool{
        self.nominator * other.denominator == other.nominator * self.denominator
    }
}
impl Eq for Fraction{}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn fraction_addicion_denom_eq_gcd(){
        let frac = Fraction{nominator: 1, denominator: 5};
        let frac2 = Fraction{nominator: 3, denominator: 15};
        let result = Fraction{nominator: 2, denominator: 5};
        assert_eq!(frac.add(&frac2), result);
    }

    #[test]
    fn fraction_addicion(){
        let frac = Fraction{nominator: 1, denominator: 6};
        let frac2 = Fraction{nominator: 4, denominator: 9};
        let result = Fraction{nominator: 11, denominator: 18};
        assert_eq!(frac.add(&frac2), result);
    }

    #[test]
    fn fraction_addicion_negative_numbers(){
        let frac = Fraction{nominator: -1, denominator: 6};
        let frac2 = Fraction{nominator: -4, denominator: 9};
        let result = Fraction{nominator: -11, denominator: 18};
        assert_eq!(frac.add(&frac2), result);
    }

    #[test]
    fn multiplication(){
        let frac = Fraction{nominator: -1, denominator: 6};
        let frac2 = Fraction{nominator: -4, denominator: 9};
        let result = Fraction{nominator: 2, denominator: 27};
        assert_eq!(frac.mult(&frac2), result);
    }

    #[test]
    fn division(){
        let frac = Fraction{nominator: 11, denominator: 20};
        let frac2 = Fraction{nominator: 110, denominator: 200};
        let result = Fraction{nominator: 1, denominator: 1};
        assert_eq!(frac.div(&frac2), result);
    }

    #[test]
    fn equality(){
        let frac = Fraction{nominator: 11, denominator: 20};
        let frac2 = Fraction{nominator: 110, denominator: 200};
        assert!(frac == frac2);
    }
    #[test]
    fn inversion(){
        let frac = Fraction{nominator: 11, denominator: 20};
        let frac2 = Fraction{nominator: 200, denominator: 110};
        assert_eq!(frac2.inverse(), frac);
    }

    #[test]
    fn power(){
        let frac = Fraction{nominator: 2, denominator: 4};
        let result = Fraction{nominator: 1, denominator: 8};
        assert_eq!(frac.pow(3), result);
    }
}