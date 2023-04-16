use std::ops;
mod euclid;

enum Number{
    Fractional(Fraction),
    Decimal(f32)
}
#[derive(PartialEq,Debug)]
struct Fraction{
    nominator : i32,
    denominator : i32
}

impl Fraction{
    fn add(&self, rhs : &Fraction) -> Fraction {
        let gcd = euclid::gcd(self.denominator, rhs.denominator);
        let left_nom : i32 = self.nominator * (rhs.denominator / gcd);
        let right_nom : i32 = rhs.nominator * (self.denominator / gcd);
        let denom : i32 = i32 :: abs (self.denominator * rhs.denominator) / gcd;
        let gcd_nom_denom = euclid :: gcd(left_nom + right_nom, denom);
        Fraction{nominator : (left_nom + right_nom) / gcd_nom_denom, denominator : denom / gcd_nom_denom}
    }
}

impl ops::Add<Number> for Number{
    type Output = Number;
    fn add(self, rhs : Number) -> Number {
        match (self, rhs) {
            (Number::Fractional(comp_first), Number::Fractional(comp_second)) =>{
                Number::Fractional(Fraction{nominator : 1, denominator : 1})
            }   
            _ => Number::Decimal(1.0)  
        }
    }
}

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
}