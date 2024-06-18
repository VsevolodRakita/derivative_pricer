//! #Introduction
//! 
//! This library provides tools for pricing derivative secureties in a Black-Scholes setting.
//! 
//! # Features
//! 
//! - [x] Black Scholes pricing formulas for european call and put options, digital call and put options, forward price of a stock,
//!     and zero coupon bonds.
//! - [x] Monte-Carlo pricer for vanilla options.
//! 
//! 

pub mod random_number_generator;
pub mod utils;
pub mod option;
pub mod statistics_gatherer;
pub mod monte_carlo_pricer;
pub mod formulas;
pub mod stock;
pub mod raw_formulas;

