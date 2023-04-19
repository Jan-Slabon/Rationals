use std::ops;
mod euclid;
mod fraction;
use fraction::Fraction;
#[derive(Debug,Copy,Clone)]
enum Number{
    Fractional(Fraction),
    Decimal(f32)
}
impl PartialEq for Number{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Fractional(comp_first), Number::Fractional(comp_second)) =>{
                comp_first == comp_second
            }   
            (Number::Fractional(frac), Number::Decimal(dec)) =>{
                (frac.nominator as f32) / (frac.denominator as f32) == *dec
            } 
            (Number::Decimal(dec), Number::Fractional(frac)) =>{
                (frac.nominator as f32) / (frac.denominator as f32) == *dec
            } 
            (Number::Decimal(a), Number::Decimal(b)) =>{
                *a==*b
            }
        }
    }
}
impl Eq for Number {}

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

impl ops::Mul<Number> for Number{
    type Output = Number;
    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Fractional(comp_first), Number::Fractional(comp_second)) =>{
                Number::Fractional(comp_first.mult(&comp_second))
            }   
            (Number::Fractional(frac), Number::Decimal(dec)) =>{
                Number::Decimal((frac.nominator as f32) / (frac.denominator as f32) * dec)
            } 
            (Number::Decimal(dec), Number::Fractional(frac)) =>{
                Number::Decimal((frac.nominator as f32) / (frac.denominator as f32) * dec)
            } 
            (Number::Decimal(a), Number::Decimal(b)) =>{
                Number::Decimal(a*b)
            }
        }
    }
}

impl ops::Div<Number> for Number{
    type Output = Number;
    fn div(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Fractional(comp_first), Number::Fractional(comp_second)) =>{
                Number::Fractional(comp_first.div(&comp_second))
            }   
            (Number::Fractional(frac), Number::Decimal(dec)) =>{
                Number::Decimal((frac.nominator as f32) / (frac.denominator as f32) / dec)
            } 
            (Number::Decimal(dec), Number::Fractional(frac)) =>{
                Number::Decimal(dec / ((frac.nominator as f32) / (frac.denominator as f32)))
            } 
            (Number::Decimal(a), Number::Decimal(b)) =>{
                Number::Decimal(a/b)
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn number_addicion_fractional(){
        let frac_1 = Number::Fractional(Fraction{nominator : 2, denominator : 3});
        let frac_2 = Number::Fractional(Fraction{nominator : 1, denominator : 3});
        let res = Number::Fractional(Fraction{nominator : 1, denominator : 1});
        assert_eq!(frac_1 + frac_2, res);
    }

    #[test]
    fn number_addicion_fractional_decimal(){
        let frac_1 = Number::Fractional(Fraction{nominator : 2, denominator : 3});
        let frac_2 = Number::Decimal(1./3.);
        let res = Number::Decimal(2./3. + 1./3.);
        assert_eq!(frac_1 + frac_2, res);
    }

    #[test]
    fn number_equality_fractions(){
        let frac_1 = Number::Fractional(Fraction{nominator : 2, denominator : 6});
        let frac_2 = Number::Fractional(Fraction{nominator : 1, denominator : 3});
        assert!(frac_1 == frac_2);
    }

    #[test]
    fn number_equality_fraction_decimal(){
        let frac_1 = Number::Fractional(Fraction{nominator : 2, denominator : 6});
        let frac_2 = Number::Decimal(2./6.);
        assert!(frac_1 == frac_2);
    }

    #[test]
    fn number_equality_decimal(){
        let frac_1 = Number::Decimal(1./3.);
        let frac_2 = Number::Decimal(2./6.);
        assert!(frac_1 == frac_2);
    }

    #[test]
    fn number_mult_fractional(){
        let frac_1 = Number::Fractional(Fraction{nominator : 2, denominator : 3});
        let frac_2 = Number::Fractional(Fraction{nominator : 1, denominator : 3});
        let res = Number::Fractional(Fraction{nominator : 2, denominator : 9});
        assert_eq!(frac_1 * frac_2, res);
    }

    #[test]
    fn number_mult_fractional_decimal(){
        let frac_1 = Number::Fractional(Fraction{nominator : 2, denominator : 3});
        let frac_2 = Number::Decimal(2.);
        let res = Number::Decimal(4./3.);
        assert_eq!(frac_1 * frac_2, res);
    }

    #[test]
    fn number_division_fractional_decimal(){
        let frac_1 = Number::Fractional(Fraction{nominator : 2, denominator : 3});
        let frac_2 = Number::Decimal(2.);
        let res = Number::Decimal(1./3.);
        let res2 = Number::Decimal(3.);
        assert_eq!(frac_1 / frac_2, res);
        assert_eq!(frac_2 / frac_1, res2);
    }
}