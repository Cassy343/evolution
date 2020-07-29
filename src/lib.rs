#![forbid(rust_2018_idioms)]
#![warn(missing_docs)]
#![allow(incomplete_features)] // Allow GATs
#![feature(generic_associated_types, associated_type_defaults)]

//! This crate provides a simple, easy-to-use API for applying the genetic algorithm to a wide range
//! of problems.

mod bits;
mod chromosome;
mod collection;
mod env;
mod individual;
mod population;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chromosome_test() {
        use chromosome::Ordered;
        use population::Population;
        use individual::OwnedGenome;
        use env::*;
        use rand::prelude::*;
        use rand::distributions::Uniform;

        let mut pop: Population<OwnedGenome<(f64, f64)>, _> = Population::with_loss(
            40,
            EnvSettings::default(),
            |indv: &OwnedGenome<(f64, f64)>| {
                let x = indv.genome.0;
                let y = indv.genome.1;
                let h = x * x - y * y;
                let k = 1.0 - x;
                100.0 * h * h + k * k
            }
        );
        
        let distr = Uniform::new(-2.0f64, 2.0f64);
        pop.initialize(|rng| {
            (
                distr.sample(&mut *rng),
                distr.sample(&mut *rng)
            )
        });
        
        pop.evolve_until(0.001);
        println!("{}", pop.loss());
    }
}
