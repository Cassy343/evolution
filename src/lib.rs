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

        let mut pop: Population<OwnedGenome<[Ordered<i32>; 5]>, _> = Population::with_loss(
            40,
            EnvSettings::default(),
            |indv: &OwnedGenome<[Ordered<i32>; 5]>| {
                indv.genome.iter().map(|ch| ch.value).sum::<i32>().abs() as f64
            }
        );

        let distr = Uniform::new(-5, 5);
        pop.initialize(|rng| {
            [
                Ordered::weighted(distr.sample(&mut *rng), 3.0),
                Ordered::weighted(distr.sample(&mut *rng), 3.0),
                Ordered::weighted(distr.sample(&mut *rng), 3.0),
                Ordered::weighted(distr.sample(&mut *rng), 3.0),
                Ordered::weighted(distr.sample(&mut *rng), 3.0)
            ]
        });

        pop.evolve_until(0.001);
        println!("{}", pop.loss());
    }
}
