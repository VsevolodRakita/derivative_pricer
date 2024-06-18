//! Implements a struct representing a stock.

use crate::utils::NonNegativeFloat;

///A struct representing a stock.
#[derive(Clone, Copy, Debug)]
pub struct Stock{
    ///The current price of the stock.
    price: NonNegativeFloat,
    ///The volatility of the stock.
    volatility: NonNegativeFloat,
    ///The rate at which the stock pays out dividents.
    divident_rate: NonNegativeFloat,
}

impl Stock {
    ///Returns a new stock with given parameters.
    pub fn new(price: NonNegativeFloat, volatility: NonNegativeFloat,divident_rate: NonNegativeFloat) ->Stock{
        Stock{
            price,
            volatility,
            divident_rate,
        }
    }
    
    pub fn get_price(&self)->NonNegativeFloat{
        self.price
    }

    pub fn get_volatility(&self)->NonNegativeFloat{
        self.volatility
    }

    pub fn get_divident_rate(&self)->NonNegativeFloat{
        self.divident_rate
    }
    

    ///Evolves the stocks price according to geometrical Brownian motion. gaussian_sample is a sample from astandard Gauusian distribution,
    /// r is the short rate of interest and time is the time.
    pub fn evolve(&mut self, gaussian_sample: f64, r: f64, time: NonNegativeFloat){
        let root_of_time = f64::from(time).sqrt();
        let half_sigma_squared = 0.5*f64::from(self.volatility)*f64::from(self.volatility);
        let exponent = (r-f64::from(self.divident_rate)-half_sigma_squared)*f64::from(time)+gaussian_sample*root_of_time*f64::from(self.volatility);
        let moved_spot=f64::from(self.price)*exponent.exp();
        self.price = NonNegativeFloat::from(moved_spot);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn stock_test1(){
        let mut s = Stock::new(NonNegativeFloat::from(10.5), NonNegativeFloat::from(0.2), NonNegativeFloat::from(0.0));
        s.evolve(-0.1, 0.05, NonNegativeFloat::from(1.0));
        assert_eq!(10.605526754383764,f64::from(s.get_price()));
    }

}