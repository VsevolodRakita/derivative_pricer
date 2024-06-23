//! Provides Black-Scholes formulas for various securities and greeks.
//! Provides Black-Scholes formulas for european call and put options, digital call and put options,
//! forward prices and zero coupon bonds, and greeks of call and put options.
//! 
//! Note: the functions in this module use the custome types `Stock` and `NonNegativeFloat` defined in `stock.rs` and `utils.rs`, respectively.
//! For ease of use, the formulas are also implemented using only the `f64` type in the module `raw_formulas`.

use crate::raw_formulas;
use crate::utils::NonNegativeFloat;
use crate::stock::Stock;

pub fn european_call_option_price(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::european_call_option_price(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn european_put_option_price(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::european_put_option_price(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn forward_price(stock: Stock, r: f64, time: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::forward_price(f64::from(stock.get_price()), r, f64::from(time), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn digital_call_price(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::digital_call_price(f64::from(stock.get_price()), 
    f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn digital_put_price(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::digital_put_price(f64::from(stock.get_price()), 
    f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn zero_coupon_bond(r: f64,time_to_maturity: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::zero_coupon_bond(r, f64::from(time_to_maturity));
    NonNegativeFloat::from(ret)
}

pub fn call_delta(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::call_delta(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn call_gamma(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::call_gamma(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn call_vega(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::call_vega(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn call_rho(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::call_rho(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn call_theta(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::call_theta(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn put_delta(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::put_delta(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn put_gamma(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::put_gamma(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn put_vega(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::put_vega(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn put_rho(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::put_rho(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}

pub fn put_theta(stock: Stock, strike:NonNegativeFloat, r: f64, time_to_expiry: NonNegativeFloat) -> NonNegativeFloat{
    let ret = raw_formulas::put_theta(f64::from(stock.get_price()), 
        f64::from(strike), r, f64::from(time_to_expiry), f64::from(stock.get_volatility()), f64::from(stock.get_divident_rate()));
    NonNegativeFloat::from(ret)
}