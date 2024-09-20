//! Provides struct representing derivative options.

use crate::stock::{GeometricBrownianMotionStock, StockState};
use crate::utils::{NonNegativeFloat, TimeStamp};
use std::rc::Rc;


/// A trait indicating that the implementing struct is a state of the underlying of some option.
/// For example, this can be the value of a stock at a certain timestamp, or the temprature at a
/// certain location at a certain timestamp.
pub trait Underlying{

}

impl Underlying for GeometricBrownianMotionStock {
    
}

/// A trait indicating that the class implementing it is an option that can be priced
pub trait DerivativeOption<T: Underlying> {
    /// Returns the time to expiry of the option, or None if the option expired.
    fn get_time_to_expiry(&self)->Option<TimeStamp>;
    /// Returns the number of random samples needed to price one path of the option.
    fn get_dimensionality(&self)->usize;
    /// Prices the option (not discounted) given one path of the underlying.
    /// #Parameters
    /// - `random_samples` - a vector of iid random samples of length `self.get_dimensionality()` from whatever distribution the option needs.
    /// - `r` - the short rate of interest.
    fn price_path(&self, random_samples: &Vec<f64>,r: f64)->f64;
}

/// A struct implementing a vanilla option, i.e. an option whose payoff only depends on the value of the underlying
/// asset at exercise time.
pub struct VanillaStockOption{
    ///A shared reference to the underlying stock.
    underlying_stock: Rc<GeometricBrownianMotionStock>,
    /// The time of expiry.
    expiry: TimeStamp,
    /// The payoff function of the option. Gets the value of the underlying asset at exercise time and a boxed vector of
    /// parameters such as strike price.
    payoff_function: Box<dyn Fn(NonNegativeFloat, &Box<Vec<f64>>)->f64>,
    /// A boxed vector of whatever parameters are needed to compute the payoff function, e.g. strike price.
    params: Box<Vec<f64>>,
}

impl VanillaStockOption {
    /// Returns a new vanilla option.
    /// # Parameters
    /// - `underlying_stock`: A shared reference to the underlying stock.
    /// - `expiry`: The expiry time.
    /// - `payoff_function`: A boxed payoff function. The function gets the value of the underlying asset at exercise time and a boxed vector of parameters such as strike price.
    /// - `params`: A boxed vector of parameters, for the payoff function.
    pub fn new(underlying_stock: &Rc<GeometricBrownianMotionStock>, expiry:TimeStamp, payoff_function: Box<dyn Fn(NonNegativeFloat, &Box<Vec<f64>>)->f64>, params: Box<Vec<f64>>)->VanillaStockOption{
        VanillaStockOption{
            underlying_stock: Rc::clone(&underlying_stock),
            expiry,
            payoff_function,
            params,
        }

    }

    /// Returns the expiry of the option.
    pub fn get_expiry(&self) -> TimeStamp{
        self.expiry
    }

    /// Returns the underlying stock of the option.
    pub fn get_underlying(&self) -> Rc<GeometricBrownianMotionStock>{
        self.underlying_stock.clone()
    }
}

impl DerivativeOption<GeometricBrownianMotionStock> for VanillaStockOption {
    ///Returns the time to expiry of the option, where the current time is considered to be the current time of the underlying stock.
    fn get_time_to_expiry(&self)->Option<TimeStamp> {
        let x=f64::from(self.expiry)-f64::from(self.underlying_stock.get_current_state().get_time());
        if x<0.0{
            return None;
        }
        Some(NonNegativeFloat::from(x))
    }
    
    /// Returns the number of random samples needed to price one path of the option.
    fn get_dimensionality(&self)->usize {
        1
    }
    
    /// Prices the option (not discounted) given one path of the underlying.
    /// #Parameters
    /// - `random_samples` - a vector with (at least...) one Gaussian sample.
    /// - `r` - the short rate of interest.
    fn price_path(&self, random_samples: &Vec<f64>, r: f64)->f64 {
        if random_samples.len()< 1{
            panic!("Incorrect length of random_samples");
        }
        if self.expiry < self.underlying_stock.get_current_state().get_time(){
            panic!("The option expiered!")
        }
        let time_stamps=vec![self.expiry];
        let state=self.underlying_stock.generate_risk_neutral_path_from_time_stamps(random_samples, &time_stamps, r);
        (self.payoff_function)(state[0].get_value(), &self.params)
    }
    
}

pub struct AsianOption{
    ///A shared reference to the underlying stock.
    underlying_stock: Rc<GeometricBrownianMotionStock>,
    /// The time of expiry.
    expiry: TimeStamp,
    /// A vector of the times at which the value of the underlying stock will be used for the average.
    monitoring_times: Vec<TimeStamp>,
    /// A vector of states of the underlying stock.
    history: Vec<StockState>,
    /// A boxed function that gets a vector of states of the underlying stock and a vector of monitoring times, and computes an average.
    average_function: Box<dyn Fn(&Vec<StockState>, &Vec<TimeStamp>)->NonNegativeFloat>,
    /// A boxed function that gets the average of the underlying stock, as computed by `self.average_function` and a boxed vector of parameters, and evaluates the payoff of the option.
    payoff_function: Box<dyn Fn(NonNegativeFloat, &Box<Vec<f64>>)->f64>,
    /// A boxed vector of whatever parameters are needed to compute the payoff function, e.g. strike price.
    params: Box<Vec<f64>>,
    
}


impl AsianOption{
    /// Returnes a new Asian option.
    /// # Parameters:
    /// - `underlying_stock`: A shared reference to the underlying stock.
    /// - `expiry`: The expiry time.
    /// - `monitoring_times`: A vector of the times at which the value of the underlying stock will be used for the average. Needs to be sorted with unique values. 
    /// - `average_function`: A boxed function that gets a vector of states of the underlying stock and a vector of monitoring times, and computes an average.
    /// - `payoff_function`: A boxed payoff function. The function gets the value of the underlying asset at exercise time and a boxed vector of parameters such as strike price.
    /// - `params`: A boxed vector of parameters, for the payoff function.
    pub fn new(underlying_stock: &Rc<GeometricBrownianMotionStock>, expiry: TimeStamp, monitoring_times: &Vec<TimeStamp>, average_function: Box<dyn Fn(&Vec<StockState>, &Vec<TimeStamp>)->NonNegativeFloat>,
        payoff_function: Box<dyn Fn(NonNegativeFloat, &Box<Vec<f64>>)->f64>, params: Box<Vec<f64>>,)->AsianOption{
            AsianOption{
                underlying_stock: underlying_stock.clone(),
                expiry,
                monitoring_times: monitoring_times.clone(),
                history: vec![underlying_stock.get_current_state()],
                average_function,
                payoff_function,
                params,
            }
        }
    
    /// Updates the option with the current state of the underlying stock.
    pub  fn update(&mut self){
        if self.history[self.history.len()-1].get_time() == self.underlying_stock.get_current_state().get_time() {
            return;
        }
        self.history.push(self.underlying_stock.get_current_state());
    }
}

impl DerivativeOption<GeometricBrownianMotionStock> for AsianOption {
    /// Returns the time to expiry of the option, or None if the option expiered.
    fn get_time_to_expiry(&self)->Option<TimeStamp> {
        let x=f64::from(self.expiry)-f64::from(self.underlying_stock.get_current_state().get_time());
        if x<0.0{
            return None;
        }
        Some(NonNegativeFloat::from(x))
    }
    
    /// Returns the number of random samples needed to price one path of the option.
    fn get_dimensionality(&self)->usize {
        let mut i=0;
        let current_time = self.underlying_stock.get_current_state().get_time();
        while i<self.monitoring_times.len() && self.monitoring_times[i]< current_time{
            i+=1;
        }
        self.monitoring_times.len()-i
    }
    
    /// Prices the option (not discounted) given one path of the underlying.
    /// #Parameters
    /// - `random_samples` - a vector of iid random samples of length `self.get_dimensionality()` from whatever distribution the option needs.
    /// - `r` - the short rate of interest.
    fn price_path(&self, random_samples: &Vec<f64>, r: f64) ->f64{
        let mut history = self.history.clone();
        if self.underlying_stock.get_current_state().get_time()!=history[history.len()-1].get_time(){
            history.push(self.underlying_stock.get_current_state());
        }
        let t0=history[history.len()-1].get_time();
        let mut time_stamps=Vec::new();
        for t in self.monitoring_times.iter(){
            if *t>t0{
                time_stamps.push(*t);
            }
        }
        let mut v=self.underlying_stock.generate_risk_neutral_path_from_time_stamps(random_samples, &time_stamps, r);
        history.append(&mut v);
        (*self.payoff_function)((*self.average_function)(&history, &self.monitoring_times), &self.params)
    }
    

}


