use std::{collections::HashSet, num::NonZeroUsize, str::FromStr};
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

/// Custom error for breeder-related things
#[derive(Debug, Clone)]
pub struct BreedingError {
    msg: String,
}

impl std::error::Error for BreedingError {}

impl std::fmt::Display for BreedingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

/// Custom error for percentage-related things.
#[derive(Debug, Clone)]
pub struct PercentageError {
    msg: String,
}

impl std::error::Error for PercentageError {}

impl std::fmt::Display for PercentageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

/// Represents a percentage in decimal format (e. g., 0.7 is 70%)
#[derive(Debug, Clone, Copy)]
pub struct Percentage(f64);

impl Percentage {
    /// Receives a number in decimal format. Will throw an error if not in the
    /// range [0.0, 1.0].
    pub fn new(n: f64) -> Result<Self, PercentageError> {
        if (0.0..=1.0).contains(&n) {
            Ok(Self(n))
        } else {
            Err(PercentageError {
                msg: format!("{} is not in the [0.0, 1.0] range", n),
            })
        }
    }

    /// Receives a number in decimal format, but will not check boundaries.
    pub unsafe fn new_unchecked(n: f64) -> Self {
        Self(n)
    }

    /// Gets the value as a simple f64.
    pub fn get(&self) -> f64 {
        self.0
    }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Percentage {
    type Err = PercentageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let float = s.parse::<f64>().map_err(|_err| PercentageError {
            msg: format!("failed parsing into a float: {}", s),
        })?;

        Percentage::new(float)
    }
}

/// Represents an offspring, which contains the text itself and a calculated
/// fitness in relation to it's parent.
#[derive(Debug)]
struct Offspring {
    text: String,
    fitness: usize,
}

impl Clone for Offspring {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            fitness: self.fitness.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.text.clone_from(&source.text);
        self.fitness = source.fitness;
    }
}

/// A struct to "breed" sentences.
#[derive(Debug, Clone)]
pub struct Breeder<R, F>
where
    R: Rng,
    F: Fn(&str, &str) -> usize,
{
    rng: R,
    alphabet: Vec<String>,
    target: String,
    offspring_per_generation: usize,
    winning_offspring_buff: Offspring,
    new_offspring_buff: Offspring,
    mutation_rate: f64,
    fitness_function: F,
}

/// Helper function to create the new individual
fn do_breed<R, F>(
    rng: &mut R,
    alphabet: &[String],
    mutation_rate: f64,
    target: &str,
    individual: &str,
    offspring_buff: &mut Offspring,
    fitness_function: &F,
) where
    R: Rng,
    F: Fn(&str, &str) -> usize,
{
    offspring_buff.text.clear();

    for g in individual.graphemes(true) {
        let next_grapheme = if rng.gen_range(f64::MIN_POSITIVE..=1.0) <= mutation_rate {
            alphabet.iter().choose(rng).unwrap()
        } else {
            g
        };

        offspring_buff.text.push_str(next_grapheme);
    }

    offspring_buff.fitness = (fitness_function)(target, &offspring_buff.text);
}

impl<R, F> Breeder<R, F>
where
    R: Rng,
    F: Fn(&str, &str) -> usize,
{
    /// Create a new Breeder.
    ///
    /// `rng` is a random number generator as defined by the [`Rng`] trait.
    /// `target` is the target phrase we wish to reach. `alphabet` is a string
    /// containing all graphemes that will be used to create offspring.
    /// `offsprint_per_generation` says how many offspring will be made each
    /// generation. `mutation_rate` indicates the probability that, during copy,
    /// a random mutation will occur (a random grapheme be selected from the
    /// alphabet, instead of a copy). `fitness_function` is a function that determines
    /// the fitness of the offspring.
    ///
    /// `fitness_function(&str, &str)` receives the target string and the offspring to evaluate,
    /// in that order.
    ///
    /// The alphabet will be NFC-normalized, separated by grapheme clusters,
    /// and repeated graphemes will be eliminated.
    pub fn new(
        rng: R,
        target: String,
        alphabet: &str,
        offspring_per_generation: NonZeroUsize,
        mutation_rate: Percentage,
        fitness_function: F,
    ) -> Result<Self, BreedingError> {
        let alphabet: Vec<String> = alphabet
            .nfc()
            .collect::<String>()
            .graphemes(true)
            .map(|g| g.to_string())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        if alphabet.len() == 0 {
            return Err(BreedingError {
                msg: format!("alphabet is empty"),
            });
        }

        Ok(Self {
            rng,
            alphabet,
            winning_offspring_buff: Offspring {
                text: String::with_capacity(target.len()),
                fitness: 0,
            },
            new_offspring_buff: Offspring {
                text: String::with_capacity(target.len()),
                fitness: 0,
            },
            target,
            offspring_per_generation: offspring_per_generation.get(),
            mutation_rate: mutation_rate.get(),
            fitness_function,
        })
    }

    /// Get this Breeder's alphabet.
    pub fn alphabet(&self) -> &[String] {
        &self.alphabet
    }

    /// Get this Breeder's target.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Do a breeding round, based on the graphemes of the `individual`.
    ///
    /// No normalization is made of the individual.
    pub fn breed(&mut self, individual: &str) -> (String, usize) {
        let mut current_offspring = 1;
        do_breed(
            &mut self.rng,
            &self.alphabet,
            self.mutation_rate,
            &self.target,
            individual,
            &mut self.winning_offspring_buff,
            &self.fitness_function,
        );

        while current_offspring < self.offspring_per_generation {
            do_breed(
                &mut self.rng,
                &self.alphabet,
                self.mutation_rate,
                &self.target,
                individual,
                &mut self.new_offspring_buff,
                &self.fitness_function,
            );

            if self.new_offspring_buff.fitness > self.winning_offspring_buff.fitness {
                self.winning_offspring_buff
                    .clone_from(&self.new_offspring_buff);
            }

            current_offspring += 1;
        }

        (
            self.winning_offspring_buff.text.clone(),
            self.winning_offspring_buff.fitness,
        )
    }

    /// Create an iterator that automates breeding rounds. Returns the iterator
    /// and a random string grapheme-length equal to the Breeder's target,
    ///  to be used as "generation 0".
    ///
    /// `target_fitness` allows to optionally set a an iteration limit. If Some,
    /// iteration will cease when we get an individual of at least this target
    /// fitness; you probably want this to be the fitness of the target
    /// related to itself (`fitness_function(target, target)`). If None, the
    /// iterations will continue forever.
    pub fn iter<'b>(
        &'b mut self,
        target_fitness: Option<usize>,
    ) -> (BreedingIterator<'b, R, F>, String) {
        let graphemes_in_target = self.target.graphemes(true).count();
        let mut seed = String::with_capacity(self.target.len());
        for _ in 0..graphemes_in_target {
            seed.push_str(self.alphabet.choose(&mut self.rng).unwrap());
        }

        (
            BreedingIterator {
                breeder: self,
                target_fitness,
                ended: false,
                current_individual: seed.clone(),
            },
            seed,
        )
    }
}

/// Iterator to automate breeding rounds.
///
/// Use [Breeder's `iter()` method](Breeder::iter()) to create one.
#[derive(Debug)]
pub struct BreedingIterator<'b, R, F>
where
    R: Rng,
    F: Fn(&str, &str) -> usize,
{
    breeder: &'b mut Breeder<R, F>,
    target_fitness: Option<usize>,
    ended: bool,
    current_individual: String,
}

impl<'a, R, F> Iterator for BreedingIterator<'a, R, F>
where
    R: Rng,
    F: Fn(&str, &str) -> usize,
{
    type Item = (String, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        let (offspring, fitness) = self.breeder.breed(&self.current_individual);
        self.current_individual.clone_from(&offspring);

        match self.target_fitness {
            Some(target) => {
                if fitness >= target {
                    self.ended = true;
                }
            }
            None => (),
        }

        Some((offspring, fitness))
    }
}

/// Fitness function that simply ignores arguments and returns 1.
///
/// For tests and experiments where you want all individuals to be equally fit.
pub fn fitness_always_1(_target: &str, _offspring: &str) -> usize {
    1
}

/// Traditional fitness function.
///
/// Both `target` and `offspring` are separated into graphemes (no normalization)
/// and compared. For each grapheme of the offspring that is equal to a grapheme
/// in the target, in the same position, a point is awarded.
pub fn fitness_comparison(target: &str, offspring: &str) -> usize {
    let mut fitness = 0;
    target
        .graphemes(true)
        .zip(offspring.graphemes(true))
        .for_each(|(ind, off)| {
            if ind == off {
                fitness += 1;
            }
        });

    fitness
}
