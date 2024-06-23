//! Provides Black-Scholes formulas for various securities and greeks, with inputs being f64.
//! Provides Black-Scholes formulas for european call and put options, digital call and put options,
//! forward prices, zero coupon bonds, and the greeks of put and call options.
//! 
//! The formulas in this module do not use the custom types `NonNegativeFloat` and `Stock`, so they can be used more
//! easily outside the library.
//! All functions panic if provided with negative parameters (except for short rate of interest).

use crate::utils;

pub fn european_call_option_price(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0 {
        panic!("One of the parameters is negative")
    }
    if time_to_expiry==0.0{
        if spot > strike{
            return spot-strike;
        }
        return 0.0;
    }
    let d1 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate+0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    let d2 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate-0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    spot*utils::cumulative_normal_function(d1)*(-time_to_expiry*divident_rate).exp()-
        strike*utils::cumulative_normal_function(d2)*(-short_rate_of_interest*time_to_expiry).exp()
}

pub fn european_put_option_price(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_maturity: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_maturity < 0.0 || volatility < 0.0 || divident_rate < 0.0{
        panic!("One of the parameters is negative")
    }
    if time_to_maturity==0.0{
        if spot < strike{
            return strike-spot;
        }
        return 0.0;
    }

    let d1 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate+0.5*volatility*volatility)*time_to_maturity)/(time_to_maturity.sqrt()*volatility);
    let d2 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate-0.5*volatility*volatility)*time_to_maturity)/(time_to_maturity.sqrt()*volatility);
    strike*utils::cumulative_normal_function(-d2)*(-short_rate_of_interest*time_to_maturity).exp()-
        spot*utils::cumulative_normal_function(-d1)*(-time_to_maturity*divident_rate).exp()
}

pub fn forward_price(spot: f64, short_rate_of_interest: f64, time: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || time < 0.0 || divident_rate < 0.0{
        panic!("One of the parameters is negative")
    }
    spot*((short_rate_of_interest-divident_rate)*time).exp()
}

pub fn digital_call_price(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0 {
        panic!("One of the parameters is negative")
    }
    let d2 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate-0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    (-short_rate_of_interest*time_to_expiry).exp()*utils::cumulative_normal_function(d2)
}

pub fn digital_put_price(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0{
        panic!("One of the parameters is negative")
    }
    let d2 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate-0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    (-short_rate_of_interest*time_to_expiry).exp()*utils::cumulative_normal_function(-d2)
}

pub fn zero_coupon_bond(short_rate_of_interest: f64, time_to_maturity: f64) -> f64{
    if time_to_maturity < 0.0{
        panic!("One of the parameters is negative")
    }
    (-short_rate_of_interest*time_to_maturity).exp()
}

///returns the derivatie of a european call option with respect to the spot, i.e. the delta.
pub fn call_delta(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0 {
        panic!("One of the parameters is negative")
    }
    let d1 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate+0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    utils::cumulative_normal_function(d1)*(-divident_rate*time_to_expiry).exp()
}

///returns the second derivatie of a european call option with respect to the spot, i.e. the gamma.
pub fn call_gamma(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0 {
        panic!("One of the parameters is negative")
    }
    let d1 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate+0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    utils::normal_probability_density_function(d1)*((-divident_rate*time_to_expiry).exp())/(volatility*spot*(time_to_expiry.sqrt()))
}

///returns the derivatie of a european call option with respect to the volatility, i.e. the vega.
pub fn call_vega(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0 {
        panic!("One of the parameters is negative")
    }
    let d1 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate+0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    utils::normal_probability_density_function(d1)*spot*(time_to_expiry.sqrt())*((-divident_rate*time_to_expiry).exp())
}

///returns the derivatie of a european call option with respect to the time to expiry, i.e. the theta.
pub fn call_theta(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0 {
        panic!("One of the parameters is negative")
    }
    let d1 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate+0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    let d2 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate-0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    let t1 = spot*utils::normal_probability_density_function(d1)*volatility*(0.5/time_to_expiry.sqrt());
    let t2 = divident_rate*spot*utils::cumulative_normal_function(d1);
    let t3 = short_rate_of_interest*strike*utils::cumulative_normal_function(d2)*((-short_rate_of_interest*time_to_expiry).exp());
    (t2-t1)*(-divident_rate*time_to_expiry).exp()-t3
}

///returns the derivatie of a european call option with respect to the short rate of interest, i.e. the rho.
pub fn call_rho(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    let d2 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate-0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    strike*time_to_expiry*utils::cumulative_normal_function(d2)*(-short_rate_of_interest*time_to_expiry).exp()
}

///returns the derivatie of a european put option with respect to the spot, i.e. the delta.
pub fn put_delta(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0 {
        panic!("One of the parameters is negative")
    }
    let d1 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate+0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    (utils::cumulative_normal_function(d1)-1.0)*(-divident_rate*time_to_expiry).exp()
}

///returns the second derivatie of a european put option with respect to the spot, i.e. the gamma. Is equal to the gamma of the call option.
pub fn put_gamma(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    call_gamma(spot, strike, short_rate_of_interest, time_to_expiry, volatility, divident_rate)
}

///returns the derivatie of a european call option with respect to the volatility, i.e. the vega. Is equal to the vega of a call option.
pub fn put_vega(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    call_vega(spot, strike, short_rate_of_interest, time_to_expiry, volatility, divident_rate)
}

///returns the derivatie of a european put option with respect to the time to expiry, i.e. the theta.
pub fn put_theta(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    if spot < 0.0 || strike < 0.0 || time_to_expiry < 0.0 || volatility < 0.0 || divident_rate < 0.0 {
        panic!("One of the parameters is negative")
    }
    let d1 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate+0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    let d2 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate-0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    let t1 = spot*utils::normal_probability_density_function(d1)*volatility*(0.5/time_to_expiry.sqrt());
    let t2 = divident_rate*spot*utils::cumulative_normal_function(-d1);
    let t3 = short_rate_of_interest*strike*utils::cumulative_normal_function(-d2)*((-short_rate_of_interest*time_to_expiry).exp());
    (-t2-t1)*(-divident_rate*time_to_expiry).exp()+t3
}

///returns the derivatie of a european put option with respect to the short rate of interest, i.e. the rho.
pub fn put_rho(spot: f64, strike: f64, short_rate_of_interest: f64, time_to_expiry: f64, volatility: f64, divident_rate: f64) ->f64{
    let d2 = ((spot/strike).ln() + (short_rate_of_interest-divident_rate-0.5*volatility*volatility)*time_to_expiry)/(time_to_expiry.sqrt()*volatility);
    -strike*time_to_expiry*utils::cumulative_normal_function(-d2)*(-short_rate_of_interest*time_to_expiry).exp()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_times_zero_cupon_is_spot(){
        assert!((32.333-forward_price(32.333, 4.657, 3.2345, 0.0)*zero_coupon_bond(4.657, 3.2345))
            .abs()<1e-14);
    }

    #[test]
    fn call_put_parity_test_1(){
        assert!((zero_coupon_bond(3.33, 0.15)*10.111-european_put_option_price(15.45, 10.111, 3.33, 0.15, 6.75, 0.0)-
            (15.45-european_call_option_price(15.45, 10.111, 3.33, 0.15, 6.75, 0.0))).abs()<1e-14);
    }

    #[test]
    fn call_put_parity_test_2(){
        assert!((5.0-4.0*zero_coupon_bond(2.0, 2.0)-
            (european_call_option_price(5.0, 4.0, 2.0, 2.0, 1.0, 0.0)-european_put_option_price(5.0, 4.0, 2.0, 2.0, 1.0, 0.0))).abs()<1e-14);
    }

    #[test]
    fn digital_put_plus_digital_call_equals_zero_cupon(){

        assert!((zero_coupon_bond(3.784, 5.7)-digital_call_price(45.34, 43.234, 3.784, 5.7, 11.8563, 1.0)-digital_put_price(45.34, 43.234, 3.784, 5.7, 11.8563, 1.0))
            .abs()<1e-14);
    }

    #[test]
    fn call_price_test(){
        assert!((european_call_option_price(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-2.36031028).abs()<1e-6)
    }

    #[test]
    fn call_delta_test(){
        assert!((call_delta(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-0.23812531).abs()<1e-6)
    }

    #[test]
    fn call_gamma_test(){
        assert!((call_gamma(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-0.01671937).abs()<1e-6)
    }

    #[test]
    fn call_vega_test(){
        assert!((call_vega(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-36.72893205).abs()<1e-6)
    }

    #[test]
    fn call_rho_test(){
        assert!((call_rho(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-31.08530034).abs()<1e-6)
    }

    #[test]
    fn call_theta_test(){
        assert!((call_theta(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)+2.72505217).abs()<1e-6)
    }

    #[test]
    fn put_price_test(){
        assert!((european_put_option_price(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-16.69385653).abs()<1e-6)
    }

    #[test]
    fn put_delta_test(){
        println!("{}",put_delta(101.2, 123.0, 0.07, 1.43, 0.15, 0.03));
        assert!((put_delta(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)+0.71988186).abs()<1e-6)
    }

    #[test]
    fn put_gamma_test(){
        assert!((put_gamma(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-0.01671937).abs()<1e-6)
    }

    #[test]
    fn put_vega_test(){
        println!("put vega {}",put_vega(101.2, 123.0, 0.07, 1.43, 0.15, 0.03));
        assert!((put_vega(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-36.72893205).abs()<1e-6)
    }

    #[test]
    fn put_rho_test(){
        assert!((put_rho(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)+128.05063872).abs()<1e-6)
    }

    #[test]
    fn put_theta_test(){
        assert!((put_theta(101.2, 123.0, 0.07, 1.43, 0.15, 0.03)-2.15630915).abs()<1e-6)
    }

}