use std::{error::Error, num::NonZeroUsize};

use clap::Parser;
use weasel_rs::{fitness_comparison, Breeder, Percentage};

const DEFAULT_TARGET: &str = "METHINKS IT IS LIKE A WEASEL";
const DEFAULT_OFFSPRING: usize = 100;
const DEFAULT_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz ";
const DEFAULT_MUTATION_RATE: f64 = 0.05;

/// Run the weasel algorithm from The Blind Watchmaker
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Target phrase.
    #[arg(default_value = DEFAULT_TARGET)]
    target: String,

    /// How many offspring per generation? Must be non-zero.
    #[arg(short, long, default_value_t = unsafe { NonZeroUsize::new_unchecked(DEFAULT_OFFSPRING) })]
    offspring: NonZeroUsize,

    /// Alphabet to use. Will be Unicode-normalized (NFC), separated in grapheme
    /// clusters, and repeated graphems will be eliminated.
    #[arg(short, long, default_value = DEFAULT_ALPHABET)]
    alphabet: String,

    /// Max number of generations.
    #[arg(short = 'M', long)]
    max_generations: Option<usize>,

    /// Mutation rate. Must be in the inclusive range [0.0, 1.0].
    #[arg(short, long, default_value_t = unsafe { Percentage::new_unchecked(DEFAULT_MUTATION_RATE) })]
    mutation_rate: Percentage,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut breeder = Breeder::new(
        rand::thread_rng(),
        args.target.clone(),
        &args.alphabet,
        args.offspring,
        args.mutation_rate,
        fitness_comparison,
    )?;

    let target_fitness = fitness_comparison(&args.target, &args.target);

    let (breeding_iter, seed) = breeder.iter(Some(target_fitness));
    print_offspring(
        0,
        &seed,
        fitness_comparison(&args.target, &seed),
        target_fitness,
    );

    // Create the breeder attached to a generation count
    let breeding_iter = breeding_iter
        .enumerate()
        .map(|(count, off)| (count + 1, off));

    // Conditionally create an iterator with a max number of generations
    let mut a;
    let mut b;
    let final_iter: &mut dyn Iterator<Item = (usize, (String, usize))> = match args.max_generations
    {
        Some(limit) => {
            a = breeding_iter.take(limit);
            &mut a
        }
        None => {
            b = breeding_iter;
            &mut b
        }
    };

    for (generation, (offspring, fitness)) in final_iter {
        print_offspring(generation, &offspring, fitness, target_fitness)
    }

    Ok(())
}

fn print_offspring(generation: usize, offspring: &str, fitness: usize, target_fitness: usize) {
    println!(
        "{}: {} ({}/{})",
        generation, offspring, fitness, target_fitness
    )
}
