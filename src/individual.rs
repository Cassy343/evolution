use crate::collection::IndexedCollection;

pub trait Individual {
    type Genotype: Clone + IndexedCollection;

    fn genotype(&mut self) -> Self::Genotype;

    fn set_genotype(&mut self, genotype: Self::Genotype);
}

pub struct BorrowedGenome<'a, G> {
    pub genome: &'a mut G
}

impl<'a, G: Clone + IndexedCollection> BorrowedGenome<'a, G> {
    pub fn new(genome: &'a mut G) -> Self {
        BorrowedGenome {
            genome
        }
    }
}

impl<'a, G: Clone + IndexedCollection> Individual for BorrowedGenome<'a, G> {
    type Genotype = G;

    fn genotype(&mut self) -> Self::Genotype {
        self.genome.clone()
    }

    fn set_genotype(&mut self, genotype: Self::Genotype) {
        *self.genome = genotype;
    }
}

pub trait SpawnableIndividual: Sized + Individual {
    fn spawn(genotype: Self::Genotype) -> Self;
}

pub struct OwnedGenome<G> {
    pub genome: G
}

impl<G: Clone + IndexedCollection> OwnedGenome<G> {
    pub fn new(genome: G) -> Self {
        OwnedGenome {
            genome
        }
    }
}

impl<G: Clone + IndexedCollection> Individual for OwnedGenome<G> {
    type Genotype = G;

    fn genotype(&mut self) -> Self::Genotype {
        self.genome.clone()
    }

    fn set_genotype(&mut self, genotype: Self::Genotype) {
        self.genome = genotype;
    }
}

impl<G: Clone + IndexedCollection> SpawnableIndividual for OwnedGenome<G> {
    fn spawn(genotype: Self::Genotype) -> Self {
        Self::new(genotype)
    }
}