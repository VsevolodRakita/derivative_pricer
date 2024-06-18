//! Provides Black-Scholes formulas for various securities, with inputs being f64.
//! Provides Black-Scholes formulas for european call and put options, digital call and put options,
//! forward prices and zero coupon bonds.
//! 
//! The formulas in this module do not use the custom types `NonNegativeFloat` and `Stock`, so they can be used more
//! easily outside the library.

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
}