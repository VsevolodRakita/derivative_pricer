//! Provides struct representing derivative options.
//! Currently only vanilla options are implemented.

use crate::utils::NonNegativeFloat;

/// A trait indicating that the class implementing it is an option that can be priced (I would prefer to call this trait 
/// option, but in rust this is already used for something else). The method payoff gets a pth of the underlying asset
/// and returns the payoff of the option at the time of exercise.
pub trait OptionDerivative{
    // Required method
    fn payoff(&self, path: &Vec<f64>) ->f64;
}

/// A struct implementing a vanilla option, i.e. an option whose payoff only depends on the value of the underlying
/// asset at exercise time.
pub struct VanillaOption{
    /// The payoff function of the option. Gets the value of the underlying asset at exercise time and a boxed vector of
    /// parameters such as strike price.
    payoff_function: Box<dyn Fn(f64,&Box<Vec<f64>>)->f64>,
    /// The time of expiry.
    expiry: NonNegativeFloat,
    /// A boxed vector of whatever parameters are needed to compute the payoff function, e.g. strike price.
    params: Box<Vec<f64>>,
}

impl VanillaOption {
    /// Returns a new vanilla option.
    pub fn new(payoff_function: Box<dyn Fn(f64, &Box<Vec<f64>>)->f64>, expiry: NonNegativeFloat, params: Box<Vec<f64>>) -> VanillaOption {
        VanillaOption{
            payoff_function,
            expiry,
            params,
        }
    }

    /// Returns the expiry of the option.
    pub fn get_expiry(&self) -> NonNegativeFloat{
        self.expiry
    }
}

impl OptionDerivative for VanillaOption{
    /// Computes the payoff of the function. Since this is a vanilla option, the only information requiered from the path is the last
    /// value, i.e. the value of the underlying at time of exercise.
    fn payoff(&self, path: &Vec<f64>) ->f64{
        (*self.payoff_function)(path[path.len()-1], &(self.params))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn euro_call(){
        let params = Box::new(vec![5.0]);
        let ec = VanillaOption::new(Box::new(
            |s: f64, params: &Box<Vec<f64>>| -> f64 {
                if s -params[0]> 0.0{
                    return s -params[0];
                }
                0.0
            }), NonNegativeFloat::from(7.4), params);
        let path = vec![8.9];
        assert_eq!(8.9-5.0, ec.payoff(&path));
    }

    #[test]
    fn euro_call2(){
        let params = Box::new(vec![6.7]);
        let ec = VanillaOption::new(Box::new(
            |s: f64, params: &Box<Vec<f64>>| -> f64 {
                if s -params[0]> 0.0{
                    return s -params[0];
                }
                0.0
            }), NonNegativeFloat::from(7.4), params);
        let path = vec![5.23];
        assert_eq!(0.0, ec.payoff(&path));
    }

    #[test]
    fn euro_call3(){
        let params = Box::new(vec![6.7]);
        let ec = VanillaOption::new(Box::new(
            |s: f64, params: &Box<Vec<f64>>| -> f64 {
                if s -params[0]> 0.0{
                    return s -params[0];
                }
                0.0
            }), NonNegativeFloat::from(7.4), params);
        assert_eq!(7.4, f64::from(ec.get_expiry()));
    }



}