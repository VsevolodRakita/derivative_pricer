//! Provides an interface for statistics gatherers for collecting results of Monte Carlo simulations.

///An interface for statistics gatherers.
pub trait StatisticsGathererTrait{
    ///This function collects statistics from the given `result`.
    fn dump_one_result(&mut self, result: f64);
    ///Returns the gathered results.
    fn get_results_so_far(&self) -> Vec<Vec<f64>>;
}

///A statistics gatherer that computes the mean of all gathered results.
pub struct MeanStatisticsGatherer{
    ///Sum of all results so far.
    running_sum: f64,
    ///Number of results collected.
    paths_done: usize,
}

impl MeanStatisticsGatherer {
    ///Returns a new statistics gatherer.
    pub fn new() -> MeanStatisticsGatherer{
        MeanStatisticsGatherer{
            running_sum: 0.0,
            paths_done: 0,
        }
    }
}

impl StatisticsGathererTrait for MeanStatisticsGatherer {
    ///Adds the given `result` to the gatherer.
    fn dump_one_result(&mut self, result: f64){
        self.running_sum+=result;
        self.paths_done+=1;
    }

    ///Returns the current mean of all gathered results wraped in a two dimensional `Vec`.
    fn get_results_so_far(&self) -> Vec<Vec<f64>>{
        vec![vec![(self.running_sum/self.paths_done as f64)]]
    }
}


#[cfg(test)]
mod tests {
    use crate::random_number_generator::{RandomNumberGenerator, RandomNumberGeneratorTrait};

    use super::*;

    #[test]
    fn stats_gatherer_test(){
        let mut sg = MeanStatisticsGatherer::new();
        sg.dump_one_result(4.2);
        sg.dump_one_result(2.0);
        assert_eq!(3.1,sg.get_results_so_far()[0][0]);
    }

    #[test]
    fn stats_gatherer_test2(){
        let mut sg = MeanStatisticsGatherer::new();
        sg.dump_one_result(2000.0);
        sg.dump_one_result(3000.0);
        sg.dump_one_result(1000.0);
        sg.dump_one_result(2000.0);
        sg.dump_one_result(2000.0);
        sg.dump_one_result(2000.0);
        sg.dump_one_result(2000.0);
        assert_eq!(2000.0,sg.get_results_so_far()[0][0]);
    }

    #[test]
    fn stats_gatherer_test3(){
        let mut sg = MeanStatisticsGatherer::new();
        let dim =50000000;
        let rng = RandomNumberGenerator::new(None, dim);
        let gus = rng.get_gaussians();
        for g in gus{
            sg.dump_one_result(g);
        }
        println!("{}",sg.get_results_so_far()[0][0]);
    }
}