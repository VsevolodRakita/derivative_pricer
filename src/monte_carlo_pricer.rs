//! Provides Monte Carlo pricers for various types of derivative options.
//! Currently implements a Monte Carlo pricer only for vanilla options

/* 
use crate::random_number_generator::{RandomNumberGenerator, RandomNumberGeneratorTrait};
use crate::option::{AsianOption, OptionDerivative, UnderlyingState, VanillaOption};
use crate::statistics_gatherer::StatisticsGathererTrait;
use crate::utils::{NonNegativeFloat, TimeStamp};
use crate::stock::Stock;
*/

use crate::option::{DerivativeOption, Underlying};
use crate::random_number_generator::RandomNumberGeneratorTrait;
use crate::statistics_gatherer::StatisticsGathererTrait;

/// A Monte Carlo Simulator.
/// 
/// # Parameters
/// 
/// - `option` - A `DerivativeOption`, as defined in the `option` module.
/// - `gatherer` - A mutable object implementing the `StatisticsGathererTrait` trait described in the `statistics_gatherer` module.
///     This will be used to output the results of the Monte Carlo simulation.
/// - `r` - the short rate of interest.
/// - `rng` - an object implementing the `RandomNumberGeneratorTrait`, such as `RandomNumberGenerator`. Both are descrived in the `random-number_generator` module.
/// - `number_of_paths` - The number of trials in the simulation.
/// 
/// # Panics
/// 
/// The function panics if `option.expiry - evaluation_time` is negative.
pub fn monte_carlo_simulation<T>(option: &impl DerivativeOption<T>, gatherer: &mut impl StatisticsGathererTrait, r: f64, rng: &mut impl RandomNumberGeneratorTrait, 
    number_of_paths: usize)
where T: Underlying{
    let tau= option.get_time_to_expiry().expect("The option expiered!");
    let discount_factor = f64::exp(-r*f64::from(tau));
    for _ in 0..number_of_paths{
        gatherer.dump_one_result(discount_factor*option.price_path(&rng.get_gaussians(option.get_dimensionality()), r));
    }
}

/// A function that returnes the value of the given option.
/// A wraper function for `monte_carlo_simulation` that does not require creating a statistics gatherer and random number generator.
/// 
/// # Parameters
/// 
/// - `option` - A `DerivativeOption`, as defined in the `option` module.
/// - `r` - the short rate of interest.
/// - `seed` - An optional seed for the random number generation. If `None`, a random seed will be used.
/// - `number_of_paths` - The number of trials in the simulation.
pub fn monte_carlo_pricer<T>(option: &impl DerivativeOption<T>, r: f64, seed: Option<u64>, number_of_paths: usize)->f64
where T: Underlying{
    let mut sg = crate::statistics_gatherer::MeanStatisticsGatherer::new();
    let mut rng = crate::random_number_generator::RandomNumberGenerator::new(seed);
    monte_carlo_simulation(option, &mut sg, r, &mut rng, number_of_paths);
    sg.get_results_so_far()[0][0]
}
 
#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::option::{AsianOption, VanillaStockOption};
    use crate::stock::{GeometricBrownianMotionStock, StockState};
    use crate::utils::{NonNegativeFloat, TimeStamp};

    use super::*;

    #[test]
    fn vanilla_call_test1() {
        let stock = GeometricBrownianMotionStock::new(NonNegativeFloat::from(3.2), TimeStamp::from(0.0), 
            1.0, NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.0));
        let params = Box::new(vec![5.0]);
        fn payoff(spot: NonNegativeFloat, params: &Box<Vec<f64>>)->f64{
            f64::max(f64::from(spot)-params[0], 0.0)
        }

        let opt = VanillaStockOption::new(&Rc::new(stock), TimeStamp::from(3.7), Box::new(payoff), params);
        assert!(f64::abs(monte_carlo_pricer(&opt, 0.05, None, 100000)-0.2)<0.01)
        
    }

    #[test]
    fn vanilla_call_test2() {
        let stock = GeometricBrownianMotionStock::new(NonNegativeFloat::from(3.2), TimeStamp::from(0.0), 
            1.0, NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.0));
        let params = Box::new(vec![10.0]);
        fn payoff(spot: NonNegativeFloat, params: &Box<Vec<f64>>)->f64{
            f64::max(f64::from(spot)-params[0], 0.0)
        }

        let opt = VanillaStockOption::new(&Rc::new(stock), TimeStamp::from(3.7), Box::new(payoff), params);
        assert!(f64::abs(monte_carlo_pricer(&opt, 0.05, None, 100000)-0.0)<0.01)
        
    }

    #[test]
    fn vanilla_put_test1() {
        let stock = GeometricBrownianMotionStock::new(NonNegativeFloat::from(3.2), TimeStamp::from(0.0), 
            1.0, NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.0));
        let params = Box::new(vec![5.0]);
        fn payoff(spot: NonNegativeFloat, params: &Box<Vec<f64>>)->f64{
            f64::max(params[0]-f64::from(spot), 0.0)
        }

        let opt = VanillaStockOption::new(&Rc::new(stock), TimeStamp::from(3.7), Box::new(payoff), params);
        assert!(f64::abs(monte_carlo_pricer(&opt, 0.05, None, 100000)-1.16)<0.01)
        
    }

    #[test]
    fn vanilla_put_test2() {
        let stock = GeometricBrownianMotionStock::new(NonNegativeFloat::from(3.2), TimeStamp::from(0.0), 
            1.0, NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.0));
        let params = Box::new(vec![10.0]);
        fn payoff(spot: NonNegativeFloat, params: &Box<Vec<f64>>)->f64{
            f64::max(params[0]-f64::from(spot), 0.0)
        }

        let opt = VanillaStockOption::new(&Rc::new(stock), TimeStamp::from(3.7), Box::new(payoff), params);
        assert!(f64::abs(monte_carlo_pricer(&opt, 0.05, None, 100000)-5.12)<0.01)
        
    }

    #[test]
    fn vanilla_put_test3() {
        let stock = GeometricBrownianMotionStock::new(NonNegativeFloat::from(3.2), TimeStamp::from(0.0), 
            1.0, NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.04));
        let params = Box::new(vec![10.0]);
        fn payoff(spot: NonNegativeFloat, params: &Box<Vec<f64>>)->f64{
            f64::max(params[0]-f64::from(spot), 0.0)
        }

        let opt = VanillaStockOption::new(&Rc::new(stock), TimeStamp::from(3.7), Box::new(payoff), params);
        assert!(f64::abs(monte_carlo_pricer(&opt, 0.05, None, 100000)-5.55)<0.01)
        
    }

    #[test]
    fn asian_call_test1(){
        let stock=GeometricBrownianMotionStock::new(NonNegativeFloat::from(10.2), TimeStamp::from(0.0), 
        1.0, NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.0));
        fn average(states: &Vec<StockState>,monitoring_times: &Vec<TimeStamp>)->NonNegativeFloat{
            let mut sum=0.0;
            let mut j=0;
            for t in monitoring_times.iter(){
                while j< states.len() && states[j].get_time()<*t{
                    j+=1;
                }
                if states[j].get_time()==*t{
                    sum+=f64::from(states[j].get_value());
                }
                else {
                    let a=(f64::from(states[j].get_time())-f64::from(*t))/(f64::from(states[j].get_time())-f64::from(states[j-1].get_time()));
                    sum+=a*f64::from(states[j-1].get_value())+(1.0-a)*f64::from(states[j].get_value());
                }
            }
            NonNegativeFloat::from(sum/monitoring_times.len() as f64)
        }


        fn payoff(average: NonNegativeFloat, params: &Box<Vec<f64>>)->f64{
            f64::max(f64::from(average)-params[0], 0.0)
        }
        let monitoring_times = vec![TimeStamp::from(0.0), TimeStamp::from(1.0), 
            TimeStamp::from(2.0), TimeStamp::from(3.0), TimeStamp::from(4.0), TimeStamp::from(5.0)];
        let op = AsianOption::new(&Rc::new(stock), TimeStamp::from(5.0), &monitoring_times, Box::new(average), 
            Box::new(payoff), Box::new(vec![5.4 as f64]));
        assert!(f64::abs(monte_carlo_pricer(&op, 0.03, None, 300000)-4.83)<0.01)
    }

    #[test]
    fn asian_put_test1(){
        let stock=GeometricBrownianMotionStock::new(NonNegativeFloat::from(10.2), TimeStamp::from(0.0), 
        1.0, NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.0));
        fn average(states: &Vec<StockState>,monitoring_times: &Vec<TimeStamp>)->NonNegativeFloat{
            let mut sum=0.0;
            let mut j=0;
            for t in monitoring_times.iter(){
                while j< states.len() && states[j].get_time()<*t{
                    j+=1;
                }
                if states[j].get_time()==*t{
                    sum+=f64::from(states[j].get_value());
                }
                else {
                    let a=(f64::from(states[j].get_time())-f64::from(*t))/(f64::from(states[j].get_time())-f64::from(states[j-1].get_time()));
                    sum+=a*f64::from(states[j-1].get_value())+(1.0-a)*f64::from(states[j].get_value());
                }
            }
            NonNegativeFloat::from(sum/monitoring_times.len() as f64)
        }


        fn payoff(average: NonNegativeFloat, params: &Box<Vec<f64>>)->f64{
            f64::max(params[0]-f64::from(average), 0.0)
        }
        let monitoring_times = vec![TimeStamp::from(0.0), TimeStamp::from(1.0), 
            TimeStamp::from(2.0), TimeStamp::from(3.0), TimeStamp::from(4.0), TimeStamp::from(5.0)];
        let op = AsianOption::new(&Rc::new(stock), TimeStamp::from(5.0), &monitoring_times, Box::new(average), 
            Box::new(payoff), Box::new(vec![12.6 as f64]));
        assert!(f64::abs(monte_carlo_pricer(&op, 0.03, None, 300000)-1.86)<0.01)
    }
}