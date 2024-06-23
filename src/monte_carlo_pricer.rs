//! Provides Monte Carlo pricers for various types of derivative options.
//! Currently implements a Monte Carlo pricer only for vanilla options


use crate::random_number_generator::RandomNumberGeneratorTrait;
use crate::option::{OptionDerivative, VanillaOption};
use crate::statistics_gatherer::StatisticsGathererTrait;
use crate::utils::NonNegativeFloat;
use crate::stock::Stock;


/// A Monte Carlo Simulator for vanilla options.
/// 
/// # Parameters
/// 
/// - `option` - A VanillaOption, defined in the `option` module.
/// - `gatherer` - A mutable object implementing the `StatisticsGathererTrait` trait described in the `statistics_gatherer` module.
///     This will be used to output the results of the Monte Carlo simulation.
/// - `underlying` - the underlying stock of the option.
/// - `r` - the short rate of interest.
/// - `evaluation_time` - the time at which the option is priced. The time to maturity of the option is `option.expiry - evaluation_time`. If this value
///     is negative, the function will panic (as the option already expiered).
/// - `rng` - an object implementing the `RandomNumberGeneratorTrait`, such as `RandomNumberGenerator`. Both are descrived in the `random-number_generator` module.
///     Note the number of trials in the simulation is determined by `rng.get_dimensionality()`.
/// 
/// # Panics
/// 
/// The function panics if `option.expiry - evaluation_time` is negative.
pub fn vanilla_monte_carlo_simulation( option: &VanillaOption, gatherer: &mut impl StatisticsGathererTrait,
        underlying: Stock, r: f64, evaluation_time: NonNegativeFloat, rng: &impl RandomNumberGeneratorTrait){
    let tau = f64::from(option.get_expiry())-f64::from(evaluation_time);
    if tau<0.0{
        panic!("The option expiered!")
    }
    //let rng = random_number_generator::RandomNumberGenerator::new(random_seed, number_of_paths);
    let gaussians=rng.get_gaussians();
    let discount_factor = (-r*tau).exp();
    
    for gaussian in gaussians{
        let mut underlying2=underlying.clone();
        underlying2.evolve(gaussian, r, NonNegativeFloat::from(tau));
        let path = vec![f64::from(underlying2.get_price())];
        let payoff = option.payoff(&path);
        gatherer.dump_one_result(discount_factor*payoff);
    }

}

/// A function that returnes the value of the given vanilla option.
/// A wraper function for `vanilla_monte_carlo_simulation` that does not require creating a statistics gatherer and random number generator.
/// 
/// # Parameters
/// 
/// - `option` - A VanillaOption, defined in the `option` module.
/// - `underlying` - the underlying stock of the option.
/// - `r` - the short rate of interest.
/// - `evaluation_time` - the time at which the option is priced. The time to maturity of the option is `option.expiry - evaluation_time`. If this value
///     is negative, the function will panic (as the option already expiered).
/// - `seed` -an optional seed for the random numbers in the simulation. If `None` a random seed will be used.
/// - `number_of_trials` - the number of trials for the simulation.
/// 
/// # Panics
/// 
/// The function panics if `option.expiry - evaluation_time` is negative.
pub fn monte_carlo_vanilla_pricer(option: &VanillaOption, underlying: Stock, r: f64, 
        evaluation_time: NonNegativeFloat, seed: Option<u64>, number_of_trials: usize)->NonNegativeFloat{
    let mut sg = crate::statistics_gatherer::MeanStatisticsGatherer::new();
    let rng = crate::random_number_generator::RandomNumberGenerator::new(seed, number_of_trials);
    vanilla_monte_carlo_simulation(option,&mut sg, underlying, r, evaluation_time, &rng);
    NonNegativeFloat::from(sg.get_results_so_far()[0][0])
} 


#[cfg(test)]
mod tests {
    use crate::random_number_generator::RandomNumberGenerator;
    use crate::statistics_gatherer::MeanStatisticsGatherer;
    use crate::formulas::european_call_option_price;

    use super::*;

    #[test]
    fn vanilla_pricer_test1() {
        let params = Box::new(vec![5.0]);
        let ec = VanillaOption::new(Box::new(
            |s: f64, params: &Box<Vec<f64>>| -> f64 {
                if s -params[0]> 0.0{
                    return s -params[0];
                }
                0.0
            }), NonNegativeFloat::from(7.4), params);
        let stock = Stock::new(NonNegativeFloat::from(0.0), NonNegativeFloat::from(0.0), NonNegativeFloat::from(0.0));
        let mut sg = MeanStatisticsGatherer::new();
        let rng = RandomNumberGenerator::new(None, 50);
        vanilla_monte_carlo_simulation(&ec, &mut sg, stock, 4.2, NonNegativeFloat::from(0.0), &rng);
        assert_eq!(0.0, sg.get_results_so_far()[0][0]);
    }

    #[test]
    fn vanilla_pricer_test2() {
        let params = Box::new(vec![100.0]);
        let ec = VanillaOption::new(Box::new(
            |s: f64, params: &Box<Vec<f64>>| -> f64 {
                if s -params[0]> 0.0{
                    return s -params[0];
                }
                0.0
            }), NonNegativeFloat::from(1.0), params);
        let mut sg: MeanStatisticsGatherer = MeanStatisticsGatherer::new();
        let stock = Stock::new(NonNegativeFloat::from(100.0), NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.0));
        let rng = RandomNumberGenerator::new(None, 10000000);
        vanilla_monte_carlo_simulation(&ec, &mut sg, stock, 0.05, NonNegativeFloat::from(0.0), &rng);
        let monte_carlo_prediction: f64 = sg.get_results_so_far()[0][0];
        let bs_formula = european_call_option_price(stock, NonNegativeFloat::from(100.0),0.05,NonNegativeFloat::from(1.0));
        assert!((f64::from(bs_formula) - monte_carlo_prediction).abs()<0.1);
    }

    #[test]
    fn vanilla_pricer_test3() {
        let params = Box::new(vec![100.0]);
        let ec = VanillaOption::new(Box::new(
            |s: f64, params: &Box<Vec<f64>>| -> f64 {
                if s -params[0]> 0.0{
                    return s -params[0];
                }
                0.0
            }), NonNegativeFloat::from(2.0), params);
        let mut sg: MeanStatisticsGatherer = MeanStatisticsGatherer::new();
        let stock = Stock::new(NonNegativeFloat::from(150.0), NonNegativeFloat::from(0.4), NonNegativeFloat::from(0.1));
        let rng = RandomNumberGenerator::new(None, 10000000);
        vanilla_monte_carlo_simulation(&ec, &mut sg, stock, 0.25, NonNegativeFloat::from(0.0), &rng);
        let monte_carlo_prediction = sg.get_results_so_far()[0][0];
        let bs_formula = european_call_option_price(stock,NonNegativeFloat::from(100.0), 0.25,NonNegativeFloat::from(2.0));
        assert!((f64::from(bs_formula) - monte_carlo_prediction).abs()<0.1);
    }

    #[test]
    fn vanilla_pricer_test4() {
        let params = Box::new(vec![123.0]);
        let ec = VanillaOption::new(Box::new(
            |s: f64, params: &Box<Vec<f64>>| -> f64 {
                if s -params[0]> 0.0{
                    return s -params[0];
                }
                0.0
            }), NonNegativeFloat::from(1.43), params);
        let mut sg: MeanStatisticsGatherer = MeanStatisticsGatherer::new();
        let stock = Stock::new(NonNegativeFloat::from(101.2), NonNegativeFloat::from(0.15), NonNegativeFloat::from(0.03));
        let rng = RandomNumberGenerator::new(None, 10000000);
        vanilla_monte_carlo_simulation(&ec, &mut sg, stock, 0.07, NonNegativeFloat::from(0.0), &rng);
        let monte_carlo_prediction = sg.get_results_so_far()[0][0];
        let bs_formula = european_call_option_price(stock,NonNegativeFloat::from(123.0), 0.07,NonNegativeFloat::from(1.43));
        assert!((f64::from(bs_formula) - monte_carlo_prediction).abs()<0.1);
    }

    #[test]
    fn vanilla_pricer_test5() {
        let params = Box::new(vec![123.0]);
        let ec = VanillaOption::new(Box::new(
            |s: f64, params: &Box<Vec<f64>>| -> f64 {
                if s -params[0]> 0.0{
                    return s -params[0];
                }
                0.0
            }), NonNegativeFloat::from(1.43), params);
        let stock = Stock::new(NonNegativeFloat::from(101.2), NonNegativeFloat::from(0.15), NonNegativeFloat::from(0.03));
        let monte_carlo_prediction = monte_carlo_vanilla_pricer(&ec, stock, 0.07, NonNegativeFloat::from(0.0), None, 10000000);
        let bs_formula = european_call_option_price(stock,NonNegativeFloat::from(123.0), 0.07,NonNegativeFloat::from(1.43));
        assert!((f64::from(bs_formula) - f64::from(monte_carlo_prediction)).abs()<0.1);
    }

}