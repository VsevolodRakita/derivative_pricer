//! Implements a random number generator for use in the Monte Carlo simulations.
//! A different random number generator can be implemented using the `RandomNumberGeneratorTrait` if requiered.

use rand_distr::{Normal, Distribution};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

///Provides an interface for random number generators.
pub trait RandomNumberGeneratorTrait {
    ///Returns the dimensionality of the generator, i.e. the size of the vectors of random samples returned.
    fn get_dimensionality(&self) -> usize;
    ///Returns a vector of uniform samples from [0,1] of size `get_dimensionality()`.
    fn get_uniforms(&self) ->Vec<f64>;
    ///Returns a vector of samples from the standard Gaussian distribution N(0,1) of size `get_dimensionality()`.
    fn get_gaussians(&self) -> Vec<f64>;


}


///Implements a random number generator for use in the Monte Carlo simulations.
pub struct RandomNumberGenerator{
    /// A seed for the random number generator. If none, a random seed will be used.
    seed: Option<u64>,
    /// The size of the vector of random numbers that the generator returns.
    dimensionality: usize,
}

impl RandomNumberGenerator{
    /// Returns a new random number generator with given seed and dimensionality.
    pub fn new(seed: Option<u64>, dimensionality: usize) -> RandomNumberGenerator{
        RandomNumberGenerator{seed, dimensionality}
    }
}

impl RandomNumberGeneratorTrait for RandomNumberGenerator {
    fn get_dimensionality(&self) -> usize{
        self.dimensionality
    }

    /// Returns a vector of uniform samples in (0,1) of size self.dimensionality.
    fn get_uniforms(&self) ->Vec<f64>{  
        let mut v = Vec::with_capacity(self.dimensionality);
        
        let mut r = match self.seed {
            Some(x) => StdRng::seed_from_u64(x),
            None =>  StdRng::seed_from_u64(rand::thread_rng().gen())
        };

        for _ in 0..self.dimensionality{
            v.push(r.gen());
        }
        v
    }

    /// Returns a vector of standard Gaussian samples of size self.dimensionality.
    fn get_gaussians(&self) -> Vec<f64>{  
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut v = Vec::with_capacity(self.dimensionality);
        
        let mut r = match self.seed {
            Some(x) => StdRng::seed_from_u64(x),
            None => StdRng::seed_from_u64(rand::thread_rng().gen())
        };

        for _ in 0..self.dimensionality{
            v.push(normal.sample(&mut r));
        }
        v
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_dim_test(){
        let rg = RandomNumberGenerator::new(None, 5);
        assert_eq!(5,rg.get_dimensionality());
    }

    #[test]
    fn get_gaussians_test(){
        let rg = RandomNumberGenerator::new(Some(3), 5);
        assert_eq!(vec![-0.5568042367170848, -0.5868353679798106, -0.7345823729443726, -0.14867409178273147, -0.6500355631397428],rg.get_gaussians());
    }

    #[test]
    fn get_uniforms_test(){
        let rg = RandomNumberGenerator::new(Some(7), 4);
        assert_eq!(vec![0.030317360865101395, 0.3070862833742408, 0.14264215670077263, 0.5425171865271055],rg.get_uniforms());
    }
}