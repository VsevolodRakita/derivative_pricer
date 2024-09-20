//! Implements a struct representing a stock.
use crate::utils::{NonNegativeFloat,TimeStamp};

///A struct representing a stock that satisfies the geometric Brownian motion SDE.
#[derive(Clone, Copy, Debug)]
pub struct GeometricBrownianMotionStock{
    /// The current price of the stock.
    price: NonNegativeFloat,
    /// The current time, i.e. the time at which the price was observed.
    current_time: TimeStamp,
    /// The drift of the stock.
    drift: f64,
    ///The volatility of the stock.
    volatility: NonNegativeFloat,
    ///The rate at which the stock pays out dividents.
    divident_rate: NonNegativeFloat,
}

impl GeometricBrownianMotionStock {
    ///Returns a new stock with given parameters.
    pub fn new(price: NonNegativeFloat, current_time: TimeStamp, drift: f64, volatility: NonNegativeFloat, 
                divident_rate: NonNegativeFloat) ->GeometricBrownianMotionStock{
        GeometricBrownianMotionStock{
            price,
            current_time,
            drift,
            volatility,
            divident_rate,
        }
    }
    
    ///Returns the stocks' volatility. 
    pub fn get_volatility(&self)->NonNegativeFloat{
        self.volatility
    }

    ///Returns the stocks' divident rate.
    pub fn get_divident_rate(&self)->NonNegativeFloat{
        self.divident_rate
    }

    ///Returns the stocks' drift.
    pub fn get_drift(&self)->f64{
        self.drift
    }

    ///Returns the stocks' current state, describing its current price and time stamp.
    pub fn get_current_state(&self)->StockState{
        StockState{
            value: self.price,
            time: self.current_time,
        }
    }

    /// Evolves the stocks price according to geometrical Brownian motion.
    /// # Parameters
    /// `gaussian_sample` - The gaussian_sample that will be used to evolve the stock.
    /// `time_step` - the length of time by which the stock is evolved. After calling `evolve`, the current time of the stock will be `self.current_time+time_step`
    pub fn evolve(&mut self, gaussian_sample: f64, time_step: NonNegativeFloat){
        let root_of_time = f64::from(time_step).sqrt();
        let half_sigma_squared = 0.5*f64::from(self.volatility)*f64::from(self.volatility);
        let exponent = (self.drift-f64::from(self.divident_rate)-half_sigma_squared)*f64::from(time_step)+gaussian_sample*root_of_time*f64::from(self.volatility);
        let moved_spot=f64::from(self.price)*exponent.exp();
        self.price = NonNegativeFloat::from(moved_spot);
        self.current_time = TimeStamp::from(f64::from(self.current_time)+f64::from(time_step));
    }

    /// Generates a path of the stock at the provided time stamps.
    /// Returns a vector of `StockState`, where the time stamp of each state corresponds to a time stamp in `time_stamps`.
    /// Note the path is generated under the actuall probability measure, not the risk neutral measure.
    /// # Parameters
    /// - `gaussians` - A vector of iid samples of N(0,1), i.e. the standard normal distribution. Must be the same size or larger than `time_stamps`.
    /// - `time_stamps` - A vector of time stamps. Must be strictly increasing, with the first time stamp greater or equal to `self.current_time`.
    /// # Panics
    /// - If `time_stamps` empty, not strictly increasing, or there are time stams before `self.current_time`.
    /// - If `gaussians.len()<time_stamps.len()`
    pub fn generate_path_from_time_stamps(&self, gaussians: &Vec<f64>, time_stamps: &Vec<TimeStamp>)->Vec<StockState>{
        if gaussians.len()<time_stamps.len(){
            panic!("Not enough Gaussian samples.");
        }
        if time_stamps.len()==0 || time_stamps[0]<self.current_time{
            panic!("Invalid time_stamp vector.");
        }
        let mut ans:Vec<StockState> = Vec::new();
        let mut ct = f64::from(self.current_time);
        let mut cv =f64::from(self.price);
        let half_sigma_squared = 0.5*f64::from(self.volatility)*f64::from(self.volatility);
        for i in 0..time_stamps.len(){
            let ts = time_stamps[i];
            let new_current_time = f64::from(ts);
            if new_current_time - ct < 0.0{
                panic!("Invalid time_stamp vector");
            }
            let time_step = new_current_time - ct;
            let root_of_time = (time_step).sqrt();
            let exponent = ((self.drift-f64::from(self.divident_rate)-half_sigma_squared)*time_step + gaussians[i]*root_of_time*f64::from(self.volatility)).exp();
            
            ans.push(
                StockState{
                    value: NonNegativeFloat::from(cv*exponent),
                    time: ts,
                });
            cv*=exponent;
            ct=new_current_time;
        }
        ans
    }

    /// Generates a path of the stock at the provided time stamps under the risk neutral measure.
    /// Returns a vector of `StockState`, where the time stamp of each state corresponds to a time stamp in `time_stamps`.
    /// # Parameters
    /// - `gaussians` - A vector of iid samples of N(0,1), i.e. the standard normal distribution. Must be the same size or larger than `time_stamps`.
    /// - `time_stamps` - A vector of time stamps. Must be strictly increasing, with the first time stamp greater or equal to `self.current_time`.
    /// - `r` - Short rate of interest.
    /// # Panics
    /// - If `time_stamps` empty, not strictly increasing, or there are time stams before `self.current_time`.
    /// - If `gaussians.len()<time_stamps.len()`
    pub fn generate_risk_neutral_path_from_time_stamps(&self, gaussians: &Vec<f64>, time_stamps: &Vec<TimeStamp>, r: f64)->Vec<StockState>{
        if gaussians.len()<time_stamps.len(){
            panic!("Not enough Gaussian samples.");
        }
        if time_stamps.len()==0 || time_stamps[0]<self.current_time{
            panic!("Invalid time_stamp vector.");
        }
        let mut ans:Vec<StockState> = Vec::new();
        let mut ct = f64::from(self.current_time);
        let mut cv =f64::from(self.price);
        let half_sigma_squared = 0.5*f64::from(self.volatility)*f64::from(self.volatility);
        for i in 0..time_stamps.len(){
            let ts = time_stamps[i];
            let new_current_time = f64::from(ts);
            if new_current_time - ct < 0.0{
                panic!("Invalid time_stamp vector");
            }
            let time_step = new_current_time - ct;
            let root_of_time = (time_step).sqrt();
            let exponent = ((r-f64::from(self.divident_rate)-half_sigma_squared)*time_step + gaussians[i]*root_of_time*f64::from(self.volatility)).exp();
            
            ans.push(
                StockState{
                value: NonNegativeFloat::from(cv*exponent),
                time: ts,
            });
            cv*=exponent;
            ct=new_current_time;
        }
        ans
    }

    /// Generates a path of the stock with start time `begin` and increasing by `step`.
    /// Returns a vector of `StockState`, where the time stamps start with `begin` and increase by `step` until `end`.
    /// If `inclusive` is `true`, the last time stamp will be `end`. If `inclusive` is false, the last time stamp will be strictly smaller than `end`.
    /// Note the path is generated under the actuall probability measure, not the risk neutral measure.
    /// # Parameters
    /// - `gaussians` - A vector of iid samples of N(0,1), i.e. the standard normal distribution. 
    ///     Must be same size or larger than the number of time stamps in the returned vector.
    /// - `begin` - The first time stamp of the returned path.
    /// - `step` - The size of increase in time stamps in the returned path.
    /// - `end` - The limit of time stamps.
    /// - `inclusive` - if `true`, the last time stamp in the return path will be `end`. If false, the last time stamp will be strictly smaller than `end`.
    /// 
    /// # Panics
    /// - If `begin` is smaller than self.current_time.
    /// - If `end` is smaller or equal to `begin`.
    /// - If `gausians` is not large enough.
    pub fn generate_path_from_steps(&self, gaussians: &Vec<f64>, begin: TimeStamp, step: NonNegativeFloat, end: TimeStamp, inclusive: bool)->Vec<StockState>{
        if begin < self.current_time || end < begin{
            panic!("Invalid time_stamp inputs");
        }
        let mut time_stamps = Vec::new();
        let mut ct = f64::from(begin);
        let step = f64::from(step);
        while ct < f64::from(end){
            time_stamps.push(TimeStamp::from(ct));
            ct += step;
        }
        if inclusive{
            time_stamps.push(end);
        }
        self.generate_path_from_time_stamps(gaussians, &time_stamps)
    }

    /// Generates a path of the stock under a risk neutral measure with start time `begin` and increasing by `step`.
    /// Returns a vector of `StockState`, where the time stamps start with `begin` and increase by `step` until `end`.
    /// If `inclusive` is `true`, the last time stamp will be `end`. If `inclusive` is false, the last time stamp will be strictly smaller than `end`.
    /// # Parameters
    /// - `gaussians` - A vector of iid samples of N(0,1), i.e. the standard normal distribution.
    ///     Must be same size or larger than the number of time stamps in the returned vector.
    /// - `r` - The short rate of interest.
    /// - `begin` - The first time stamp of the returned path.
    /// - `step` - The size of increase in time stamps in the returned path.
    /// - `end` - The limit of time stamps.
    /// - `inclusive` - if `true`, the last time stamp in the return path will be `end`. If false, the last time stamp will be strictly smaller than `end`.
    /// 
    /// # Panics
    /// - If `begin` is smaller than self.current_time.
    /// - If `end` is smaller or equal to `begin`.
    /// - If `gausians` is not large enough.
    pub fn generate_risk_neutral_path_from_steps(&self, gaussians: &Vec<f64>, r: f64, begin: TimeStamp, 
                                                    step: NonNegativeFloat, end: TimeStamp, inclusive: bool)->Vec<StockState>{
        if begin < self.current_time || end < begin{
            panic!("Invalid time_stamp inputs");
        }
        let mut time_stamps = Vec::new();
        let mut ct = f64::from(begin);
        let step = f64::from(step);
        while ct < f64::from(end){
            time_stamps.push(TimeStamp::from(ct));
            ct += step;
        }
        if inclusive{
            time_stamps.push(end);
        }
        self.generate_risk_neutral_path_from_time_stamps(gaussians, &time_stamps, r)
    }    



}


/// A type representing the state of a stock at some particular time. The first value  in the tuple is the stock price, 
/// and the second is the time at which it is observed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct  StockState{
    value: NonNegativeFloat, 
    time: TimeStamp,
}

impl Ord for StockState{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl  StockState {
    pub fn new(value: NonNegativeFloat, time: TimeStamp)->StockState{
        StockState{
            value,
            time
        }
    }
    
    pub fn get_value(&self) -> NonNegativeFloat{
        self.value
    }

    pub fn get_time(&self) -> TimeStamp{
        self.time
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stock_test1(){
        let mut s = GeometricBrownianMotionStock::new(NonNegativeFloat::from(5.0), TimeStamp::from(0.0), 
                1.0, NonNegativeFloat::from(0.25), NonNegativeFloat::from(0.0));
        s.evolve(1.0, NonNegativeFloat::from(0.0));
        assert_eq!(s.get_current_state(),StockState::new(NonNegativeFloat::from(5.0),TimeStamp::from(0.0)));
    }

    #[test]
    fn stock_test2(){
        let mut s = GeometricBrownianMotionStock::new(NonNegativeFloat::from(5.0), TimeStamp::from(0.0), 
                0.0, NonNegativeFloat::from(0.0), NonNegativeFloat::from(0.0));
        s.evolve(1.0, NonNegativeFloat::from(2.0));
        assert_eq!(s.get_current_state(), StockState::new(NonNegativeFloat::from(5.0),TimeStamp::from(2.0)));
    }

    #[test]
    fn stock_test3(){
        let s = GeometricBrownianMotionStock::new(NonNegativeFloat::from(5.0), TimeStamp::from(0.0), 
                0.0, NonNegativeFloat::from(0.0), NonNegativeFloat::from(0.0));
        let path = s.generate_path_from_steps(&vec![1.0;6], NonNegativeFloat::from(1.0), 
                        NonNegativeFloat::from(0.5), NonNegativeFloat::from(3.2), true);

        assert_eq!(path.len(),6);
    }

}