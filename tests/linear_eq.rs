use Rationals::Number;
use Rationals::create_number;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
// extern crate mathru as mu;
// use mathru::{
//     algebra::linear::{
//         matrix::{LUDec, Solve},
//         Matrix, Vector,
//     },
//     matrix, vector,
// };
#[test]
fn fractional_homogenus_eq(){
    let num = create_number(2, 1);
    let num2 = create_number(3, 1);
    let num3 = create_number(0, 1);
    let num4= create_number(0, 1);
    let one = create_number(1, 1);
    let one1 = create_number(1, 1);
    let m1:Array2<Number> = array![[num, num3], [num4, num2]];
    let v1:Array1<Number> = array![one, one1];
    let m2 : Array2<f32> = array![[1.,0.],[0.,1.]];
    let v2 : Array1<f32> = array![1.,0.];
    let vec = m2.solve_into(v2).unwrap();
    println!("wvctor equal {}", vec);
}