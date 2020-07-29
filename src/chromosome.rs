use crate::bits::*;
use rand::Rng;
use rand_distr::Standard;
use std::mem;

pub trait Chromosome: Sized {
    fn swap(&mut self, other: &mut Self) {
        mem::swap(self, other);
    }

    fn single_point_crossover(&mut self, other: &mut Self, rng: &mut impl Rng);

    fn point_mutation(&mut self, rng: &mut impl Rng);
}

impl Chromosome for f64 {
    fn single_point_crossover(&mut self, other: &mut Self, rng: &mut impl Rng) {
        let index = rng.gen::<usize>() % 65;
        let intermediate = self.as_bit_string().substring(0, index) ^ other.as_bit_string().substring(0, index);
        *self.as_mut_bit_string() ^= intermediate.clone();
        *other.as_mut_bit_string() ^= intermediate;
    }

    fn point_mutation(&mut self, rng: &mut impl Rng) {
        let index = rng.gen::<usize>() % 53;
        if index == 53 {
            *self = -*self;
            return;
        }

        let bit_on = self.as_mut_bit_string().flip_get(index);
        let x = *self;
        *self = x + x * (1.0 / ((1u64 << index) as f64)) * if bit_on { -1.0 } else { 1.0 };
    }
}

#[derive(Clone, Copy)]
pub struct Ordered<T> {
    pub value: T,
    weight: f32
}

impl<T: BitStringRepr> Ordered<T> {
    pub fn unweighted(value: T) -> Self {
        Ordered {
            value,
            weight: 1.0
        }
    }

    pub fn weighted(value: T, weight: f32) -> Self {
        Ordered {
            value,
            weight
        }
    }

    #[inline]
    fn weighted_bit_index(&self, rng: &mut impl Rng) -> usize {
        (rng.sample::<f32, _>(Standard).powf(self.weight) * (self.value.as_bit_string().len() as f32)) as usize
    }
}

impl<T: BitStringRepr> Chromosome for Ordered<T> {
    fn swap(&mut self, other: &mut Self) {
        mem::swap(&mut self.value, &mut other.value);
    }

    fn single_point_crossover(&mut self, other: &mut Self, rng: &mut impl Rng) {
        let index = self.weighted_bit_index(rng);
        let intermediate = self.value.as_bit_string().substring(0, index) ^ other.value.as_bit_string().substring(0, index);
        *self.value.as_mut_bit_string() ^= intermediate.clone();
        *other.value.as_mut_bit_string() ^= intermediate;
    }

    fn point_mutation(&mut self, rng: &mut impl Rng) {
        let index = self.weighted_bit_index(rng);
        self.value.as_mut_bit_string().flip(index)
    }
}