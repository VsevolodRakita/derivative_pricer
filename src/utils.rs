//! Provides various utilities.

use std::{cmp::Ordering, f64::consts::PI};


///Calculates the inverse cumulative normal function of `x`. `x` must be between 0 and 1, otherwise behaviour is undefined.
pub fn inverse_cumulative_normal_function(x: f64) -> f64{
    let a = [2.50662823884,
                        -18.61500062529,
                        41.39119773534,
                        -25.44106049637
                        ];
    let b = [-8.47351093090,
                        23.08336743743,
                        -21.06224101826,
                        3.13082909833
                        ];
    let c = [0.3374754822726147,
                        0.9761690190917186,
                        0.1607979714918209,
                        0.0276438810333863,
                        0.0038405729373609,
                        0.0003951896511919,
                        0.0000321767881768,
                        0.0000002888167364,
                        0.0000003960315187

    ];
    let y=x-0.5;
    if y.abs()<0.42{ 
        let r=y*y;
        let num = y*(a[0]+a[1]*r+a[2]*r*r+a[3]*r*r*r);
        let denom = b[0]*r+b[1]*r*r+b[2]*r*r*r+b[3]*r*r*r*r;
        return num/(1.0+denom);
    }
    let r = if y < 0.0 { x } else { 1.0 - x };
    let s = (-(r.ln())).ln();
    let mut t=c[0];
    let mut s2=s;
    for i in 1..9{
        t+=c[i]*s2;
        s2=s2*s;
    }
    if x>0.5{
        t
    }
    else{
        -t
    }
}

///Calculates the cumulative normal function at x. Output will be between 0 and 1.
pub fn cumulative_normal_function(x: f64) -> f64{
    let x2 = x.abs();
    let k = 1.0/(1.0+0.2316419*x2);
    let k2 = k*1.330274429-1.821255978;
    let k2 = k*k2+1.781477937;
    let k2 = k*k2 -0.356563782;
    let k2 = k*k2 +0.319381530;
    let n = 1.0 - k*k2*(1.0/((2.0*PI).sqrt()))*((-(x2*x2/2.0)).exp());
    if x>0.0 {
        n
    }
    else {
        1.0-n
    }
}

///Calculates the standard normal pdf.
pub fn normal_probability_density_function(x:f64)->f64{
    let sqrt_two_pi = (2.0*std::f64::consts::PI).sqrt();
    let ex = (-0.5*x*x).exp();
    ex*(1.0/sqrt_two_pi)
}

///A tuple like struct for storing non-negative f64s.
/// 
/// # Examples
/// 
/// ```
/// let x =NonNegativeFloat::from(5.5);
/// assert_eq!(5.5, f64::from(x));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct NonNegativeFloat(f64);

impl std::cmp::PartialOrd for NonNegativeFloat {
    fn partial_cmp(&self, other: &NonNegativeFloat) -> std::option::Option<std::cmp::Ordering> {
        Some(self.0.partial_cmp(&other.0).unwrap())
    }
}

impl std::cmp::PartialEq for NonNegativeFloat {
    fn eq(&self, other: &NonNegativeFloat) -> bool {
        self.0 == other.0
    }
}

impl Ord for NonNegativeFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let x = f64::from(*self);
        let y = f64::from(*other);
        if f64::abs(x-y)<1e-10{
            return Ordering::Equal;
        }
        if x < y{
            return Ordering::Less;
        }
        assert!(y < x);
        Ordering::Greater
    }
}

impl Eq for NonNegativeFloat{ }

impl From<f64> for NonNegativeFloat {
    ///Creates a new NonNegativeFloat from an f64.
    /// 
    /// #Panics
    /// 
    /// Panics if gets a negative value.
    fn from(value: f64) -> Self {
        if value < 0.0 {
            panic!("Got a negative number.")
        }
        NonNegativeFloat(value)
    }
}

impl std::fmt::Display for NonNegativeFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<NonNegativeFloat> for f64{
    fn from(value: NonNegativeFloat) -> Self {
        value.0
    }
}

pub type TimeStamp = NonNegativeFloat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cum_normal_test1() {
        println!("{}", cumulative_normal_function(1.475791028160967));
    }

    #[test]
    fn inv_cum_normal_test1() {
        println!("{}",inverse_cumulative_normal_function(0.93));
    }

    #[test]
    fn non_negative_float_test1(){
        let nnf = NonNegativeFloat::from(6.4);
        assert_eq!(6.4, f64::from(nnf));
        assert_eq!(6.4, f64::from(nnf));
    }

    #[test]
    #[should_panic]
    fn non_negative_float_test2(){
        let _nnf = NonNegativeFloat::from(-6.4);
    }

    #[test]
    fn non_negative_float_test3(){
        let nnf = NonNegativeFloat::from(0.0);
        assert_eq!(0.0, f64::from(nnf));
    }

    #[test]
    #[should_panic]
    fn non_negative_float_test4(){
        let _nnf = NonNegativeFloat::from(f64::NAN);
    }
}