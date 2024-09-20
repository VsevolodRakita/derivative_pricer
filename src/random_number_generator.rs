//! Implements a random number generator for use in the Monte Carlo simulations.
//! A different random number generator can be implemented using the `RandomNumberGeneratorTrait` if requiered.

use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

use crate::utils::inverse_cumulative_normal_function;

///Provides an interface for random number generators.
pub trait RandomNumberGeneratorTrait {
    /// Returns a vector of uniform samples from [0,1] of size `n`.
    /// Generating several random samples sequentially should yield the same result as generating them all at once.
    fn get_uniforms(&mut self, n: usize) ->Vec<f64>;
    ///Returns a vector of samples from the standard Gaussian distribution N(0,1) of size `n`.
    /// Generating several random samples sequentially should yield the same result as generating them all at once.
    fn get_gaussians(&mut self, n: usize) -> Vec<f64>;


}


///Implements a random number generator for use in the Monte Carlo simulations. A wrapper class for `StdRng`.
pub struct RandomNumberGenerator{
    rng: StdRng,
}

impl RandomNumberGenerator{
    /// Returns a new random number generator with given seed (or a random seed if `seed`=`None`).
    pub fn new(seed: Option<u64>) -> RandomNumberGenerator{
        let rng = match seed {
            Some(x) => StdRng::seed_from_u64(x),
            None =>  StdRng::seed_from_u64(rand::thread_rng().gen())
        };
        RandomNumberGenerator{rng}
    }
}

impl RandomNumberGeneratorTrait for RandomNumberGenerator {
    /// Returns a vector of uniform samples in (0,1) of size `n`.
    fn get_uniforms(&mut self, n: usize) ->Vec<f64>{  
        let mut v = Vec::with_capacity(n);

        for _ in 0..n{
            v.push(self.rng.gen());
        }
        v
    }

    /// Returns a vector of standard Gaussian samples of size `n`.
    fn get_gaussians(&mut self, n: usize) -> Vec<f64>{  
        let v = self.get_uniforms(n);
        v.into_iter().map(inverse_cumulative_normal_function).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_gaussians_test(){
        let mut rg = RandomNumberGenerator::new(Some(3));
        let mut rg2 = RandomNumberGenerator::new(Some(3));
        assert_eq!(rg2.get_gaussians(5)[3],rg.get_gaussians(5)[3]);
    }

    #[test]
    fn get_gaussians_sequential_test(){
        let mut rg = RandomNumberGenerator::new(Some(3));
        let mut rg2 = RandomNumberGenerator::new(Some(3));
        let mut v1=rg.get_gaussians(5);
        v1.append(&mut rg.get_gaussians(4));
        let v2 = rg2.get_gaussians(9);
        assert_eq!(v1, v2);
    }

    #[test]
    fn get_uniforms_test(){
        let mut rg = RandomNumberGenerator::new(Some(7));
        let mut rg2 = RandomNumberGenerator::new(Some(7));
        assert_eq!(rg2.get_uniforms(3)[1],rg.get_uniforms(3)[1]);
    }

    #[test]
    fn get_uniforms_sequential_test(){
        let mut rg = RandomNumberGenerator::new(Some(5));
        let mut rg2 = RandomNumberGenerator::new(Some(5));
        let mut v1=rg.get_uniforms(7);
        v1.append(&mut rg.get_uniforms(5));
        let v2 = rg2.get_uniforms(12);
        assert_eq!(v1, v2);
    }
}