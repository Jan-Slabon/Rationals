use std::ops;
mod euclid;
mod fraction;
use fraction::Fraction;
#[derive(Debug, PartialEq)]
enum Number{
    Fractional(Fraction),
    Decimal(f32)
}

impl ops::Add<Number> for Number{
    type Output = Number;
    fn add(self, rhs : Number) -> Number {
        match (self, rhs) {
            (Number::Fractional(comp_first), Number::Fractional(comp_second)) =>{
                Number::Fractional(comp_first.add(&comp_second))
            }   
            (Number::Fractional(frac), Number::Decimal(dec)) =>{
                Number::Decimal((frac.nominator as f32) / (frac.denominator as f32) + dec)
            } 
            (Number::Decimal(dec), Number::Fractional(frac)) =>{
                Number::Decimal((frac.nominator as f32) / (frac.denominator as f32) + dec)
            } 
            (Number::Decimal(a), Number::Decimal(b)) =>{
                Number::Decimal(a+b)
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn number_addicion(){
        let frac_1 = Number::Fractional(Fraction{nominator : 2, denominator : 3});
        let frac_2 = Number::Fractional(Fraction{nominator : 1, denominator : 3});
        let res = Number::Fractional(Fraction{nominator : 1, denominator : 1});
        assert_eq!(frac_1 + frac_2, res);
    }
}