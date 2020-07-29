use crate::chromosome::Chromosome;
use crate::collection::IndexedCollection;
use crate::env::EnvSettings;
use crate::individual::SpawnableIndividual;
use rand::{
    thread_rng,
    Rng,
    distributions::Standard,
    rngs::ThreadRng
};
use std::cmp::Ordering;

pub struct Population<I, F, R = ThreadRng> {
    settings: EnvSettings,
    individuals: Vec<I>,
    loss_func: F,
    rng: R
}

impl<I, F> Population<I, F, ThreadRng>
where
    I: SpawnableIndividual,
    F: FnMut(&I) -> f64
{
    pub fn with_loss(population_size: usize, settings: EnvSettings, loss_func: F) -> Population<I, F> {
        Population {
            settings,
            individuals: Vec::with_capacity(population_size),
            loss_func: loss_func,
            rng: thread_rng()
        }
    }
}

impl<I: SpawnableIndividual, F> Population<I, F, ThreadRng> {
    pub fn new(population_size: usize, settings: EnvSettings) -> Population<I, ()> {
        Population {
            settings: settings,
            individuals: Vec::with_capacity(population_size),
            loss_func: (),
            rng: thread_rng()
        }
    }

    pub fn initialize(&mut self, mut genotype_initializer: impl FnMut(&mut ThreadRng) -> I::Genotype) {
        for _ in 0..self.individuals.capacity() {
            self.individuals.push(I::spawn(genotype_initializer(&mut self.rng)));
        }
    }
}

impl<I, F, R> Population<I, F, R>
where
    I: SpawnableIndividual,
    <I::Genotype as IndexedCollection>::Item: Chromosome,
    F: FnMut(&I) -> f64,
    R: Rng
{
    fn mutate(&mut self, genotype: &mut I::Genotype) {
        for i in 0..genotype.size() {
            if self.rng.sample::<f32, _>(Standard) < self.settings.mutate_prob {
                genotype.get_mut(i).into_iter().for_each(|ch| ch.point_mutation(&mut self.rng));
            }
        }
    }

    pub fn evolve(&mut self) -> f64 {
        let loss = &mut self.loss_func;
        self.individuals.sort_by(|a, b| loss(a).partial_cmp(&loss(b)).unwrap_or(Ordering::Greater));
        let lowest_loss = self.individuals.get(0).map(|indv| loss(indv)).unwrap_or(0.0);

        let mut spawn_count: usize;
        if self.individuals.is_empty() {
            spawn_count = 0;
        } else {
            spawn_count = ((self.individuals.len() as f32) * self.settings.spawn_percentage) as usize;
            spawn_count = spawn_count.min(self.individuals.len() - 1) & !1;
        }
        
        if spawn_count > 0 {
            let len = self.individuals.len();
            let mut i: usize = 0;
            while i < spawn_count {
                let mut gt1 = self.individuals[i + 1].genotype();
                let mut gt2 = self.individuals[i + 2].genotype();

                assert_eq!(gt1.size(), gt2.size(), "Genotype chromosome counts did not match.");

                for i in 0..gt1.size() {
                    let ch1 = gt1.get_mut(i).unwrap();
                    let ch2 = gt2.get_mut(i).unwrap();

                    // Mutate
                    if self.rng.sample::<f32, _>(Standard) < self.settings.mutate_prob { ch1.point_mutation(&mut self.rng); }
                    if self.rng.sample::<f32, _>(Standard) < self.settings.mutate_prob { ch2.point_mutation(&mut self.rng); }

                    // Crossover
                    if self.rng.sample::<f32, _>(Standard) < self.settings.crossover_prob {
                        ch1.single_point_crossover(ch2, &mut self.rng);
                    }

                    // Swap
                    if self.rng.sample::<f32, _>(Standard) < self.settings.swap_homologous_prob {
                        ch1.swap(ch2);
                    }
                }

                self.individuals[len - i - 1] = I::spawn(gt1);
                self.individuals[len - i - 2] = I::spawn(gt2);

                i += 2;
            }
        }

        for i in 1..self.individuals.len() - spawn_count {
            let mut genotype = self.individuals[i].genotype();
            self.mutate(&mut genotype);
            self.individuals[i].set_genotype(genotype);
        }

        lowest_loss
    }

    pub fn evolve_until(&mut self, threshold: f64) {
        while self.evolve() > threshold {}
    }

    pub fn loss(&mut self) -> f64 {
        let mut loss = f64::MAX;
        let loss_fn = &mut self.loss_func;
        for indiv in self.individuals.iter() {
            let indiv_loss = loss_fn(indiv);
            if indiv_loss < loss {
                loss = indiv_loss;
            }
        }

        loss
    }

    pub fn losses(&mut self) -> Vec<f64> {
        let loss_fn = &mut self.loss_func;
        let mut losses: Vec<f64> = self.individuals.iter().map(|indiv| loss_fn(indiv)).collect();
        losses.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater));
        losses
    }
}