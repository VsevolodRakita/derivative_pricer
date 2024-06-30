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


pub struct AsianOption{
    /// The payoff function of the option. Gets a vector of the values of the underlying asset at monitoring times (this vector should be of the same 
    /// size as `monitoring_times`) and a boxed vector of parameters such as strike price.
    payoff_function: Box<dyn Fn(&Vec<f64>,&Box<Vec<f64>>)->f64>,
    /// The time of expiry.
    expiry: NonNegativeFloat,
    /// A boxed vector of whatever parameters are needed to compute the payoff function, e.g. strike price.
    params: Box<Vec<f64>>,
    /// A boxed vector of the times at which the underlying asset will be monitored. Must be monotone increasing, 
    /// with the final value smaller or equal to `expiry`.
    monitoring_times: Box<Vec<NonNegativeFloat>>,
}


impl AsianOption {
    /// Returns a new asian option.
    pub fn new(payoff_function: Box<dyn Fn(&Vec<f64>,&Box<Vec<f64>>)->f64>, expiry: NonNegativeFloat, params: Box<Vec<f64>>, 
        monitoring_times: Box<Vec<NonNegativeFloat>>) -> AsianOption {
        AsianOption{
            payoff_function,
            expiry,
            params,
            monitoring_times,
        }
    }
    /// Creates a new asian option. The monitoring times will start with `start_time` and increase by `step` until `expiry`. If
    /// `include_expiry` is true, the last element of `monitoring_times` will be equal to `expiry`, otherwise it will be strictly
    /// smaller. If `start_time`>`expiry`, `monitoring_times` will be empty.
    /// # Panics
    /// Panics if `step`=0
    pub fn new_from_time_step(payoff_function: Box<dyn Fn(&Vec<f64>,&Box<Vec<f64>>)->f64>, expiry: NonNegativeFloat, params: Box<Vec<f64>>, 
    start_time: NonNegativeFloat, step: NonNegativeFloat, include_expiry: bool) -> AsianOption {
        let mut v = Vec::new();
        let mut t = f64::from(start_time);
        let dt = f64::from(step);
        if dt==0.0{
            panic!("Step can not be 0.");
        }
        while t < f64::from(expiry){
            v.push(NonNegativeFloat::from(t));
            t+=dt
        }
        if include_expiry && start_time < expiry{
            v.push(expiry);
        }
        AsianOption{
            payoff_function,
            expiry,
            params,
            monitoring_times : Box::new(v),
        }
    }

    /// Returns the expiry of the option.
    pub fn get_expiry(&self) -> NonNegativeFloat{
        self.expiry
    }

    /// Returns the expiry of the option.
    pub fn get_monitoring_times(&self) -> Vec<NonNegativeFloat>{
        *self.monitoring_times.clone()
    }
}

impl OptionDerivative for AsianOption {
    fn payoff(&self, path: &Vec<f64>) ->f64 {
        (*self.payoff_function)(path, &self.params)
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

    #[test]
    fn asian_call(){
        let params = Box::new(vec![10.0]);
        let ac = AsianOption::new_from_time_step(Box::new(
            |s: &Vec<f64>, params: &Box<Vec<f64>>| -> f64 {
                let m = s.len();
                let x = (s.iter().sum::<f64>())/(m as f64);
                if x -params[0]> 0.0{
                    return x - params[0];
                }
                0.0
            }), NonNegativeFloat::from(10.0), params, NonNegativeFloat::from(0.0), 
                NonNegativeFloat::from(1.0),true);
        assert_eq!(10.0, f64::from(ac.get_expiry()));
    }

    #[test]
    fn asian_call2(){
        let params = Box::new(vec![10.0]);
        let ac = AsianOption::new_from_time_step(Box::new(
            |s: &Vec<f64>, params: &Box<Vec<f64>>| -> f64 {
                let m = s.len();
                let x = (s.iter().sum::<f64>())/(m as f64);
                if x -params[0]> 0.0{
                    return x - params[0];
                }
                0.0
            }), NonNegativeFloat::from(10.5), params, NonNegativeFloat::from(0.0), 
                NonNegativeFloat::from(1.0),true);
        for i in 0..11{
            assert_eq!(i as f64, f64::from(ac.get_monitoring_times()[i as usize]));
        }
        assert_eq!(10.5, f64::from(ac.get_monitoring_times()[11]));
    }

    #[test]
    fn asian_call3(){
        let params = Box::new(vec![10.0]);
        let ac = AsianOption::new_from_time_step(Box::new(
            |s: &Vec<f64>, params: &Box<Vec<f64>>| -> f64 {
                let m = s.len();
                let x = (s.iter().sum::<f64>())/(m as f64);
                if x -params[0]> 0.0{
                    return x - params[0];
                }
                0.0
            }), NonNegativeFloat::from(3.0), params, NonNegativeFloat::from(0.0), 
                NonNegativeFloat::from(1.0),true);
        let path = vec![8.0,10.0,12.0,9.0];
        assert_eq!(0.0, ac.payoff(&path));
    }
    #[test]
    fn asian_call4(){
        let params = Box::new(vec![10.0]);
        let ac = AsianOption::new_from_time_step(Box::new(
            |s: &Vec<f64>, params: &Box<Vec<f64>>| -> f64 {
                let m = s.len();
                let x = (s.iter().sum::<f64>())/(m as f64);
                if x -params[0]> 0.0{
                    return x - params[0];
                }
                0.0
            }), NonNegativeFloat::from(3.0), params, NonNegativeFloat::from(0.0), 
                NonNegativeFloat::from(1.0),true);
        let path = vec![12.0,14.0,16.0,18.0];
        assert_eq!(5.0, ac.payoff(&path));
    }



}