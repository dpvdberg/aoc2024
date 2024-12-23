use crate::solution::Solution;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test;
pub struct Day22 {}

#[derive(Clone)]
struct Buyer {
    secret_number: u64,
    count: u32,
    evolved_values: OnceCell<Vec<u64>>,
    evolved_ones: OnceCell<Vec<u32>>,
    price_changes: OnceCell<Vec<i32>>,
    strategies: OnceCell<HashSet<Vec<i32>>>
}

impl Buyer {
    fn new(secret_number: u64, count: u32) -> Buyer {
        Self {
            secret_number,
            count,
            evolved_values: OnceCell::new(),
            evolved_ones: OnceCell::new(),
            price_changes: OnceCell::new(),
            strategies: OnceCell::new(),
        }
    }

    fn evolve(&self) -> &Vec<u64> {
        self.evolved_values.get_or_init(|| {
            let mut current = self.secret_number;
            let mut results = Vec::from([current]);
            for _ in 0..self.count {
                current = Self::prune(Self::mix(current * 64, current));
                current = Self::prune(Self::mix(current / 32, current));
                current = Self::prune(Self::mix(current * 2048, current));

                results.push(current);
            }

            results
        })
    }

    fn evolve_ones(&self) -> &Vec<u32> {
        self.evolved_ones.get_or_init(|| {
            let values = self.evolve();
            values.iter().map(|&v| (v % 10) as u32).collect()
        })
    }

    fn price_changes(&self) -> &Vec<i32> {
        self.price_changes.get_or_init(|| {
            self.evolve_ones().windows(2).map(|w| w[1] as i32 - w[0] as i32).collect()
        })
    }

    fn find_strategies(&self) -> &HashSet<Vec<i32>> {
        self.strategies.get_or_init(|| {
            self.price_changes()
                .windows(4)
                .map(|w| w.to_vec())
                .collect::<HashSet<Vec<i32>>>()
        })
    }

    fn apply_strategies(&self, strategies: &HashSet<&Vec<i32>>) -> HashMap<Vec<i32>, u32> {
        let changes = self.price_changes();
        let strategy_length = strategies.iter().next().unwrap().len();
        let mut strategy_profit = strategies.iter().map(|&s| (s.clone(), 0)).collect::<HashMap<Vec<i32>, u32>>();

        for (index, window) in changes.windows(strategy_length).enumerate() {
            if let Some(w) = strategy_profit.get_mut(&window.to_vec()) {
                if *w == 0 {
                    *w = self.evolve_ones().get(index + strategy_length).unwrap().clone();
                }
            }
        }

        strategy_profit
    }

    fn prune(secret: u64) -> u64 {
        secret % 16777216
    }

    fn mix(secret: u64, input: u64) -> u64 {
        secret ^ input
    }
}

fn parse_input(input: &str) -> Vec<Buyer> {
    input.trim().lines()
        .map(|l| l.trim())
        .map(|l| Buyer::new(l.parse().unwrap(), 2000))
        .collect()
}

fn buy_strategy(buyers: Vec<Buyer>) -> u32 {
    let strategies = buyers
        .iter()
        .flat_map(|b| b.find_strategies())
        .collect::<HashSet<&Vec<i32>>>();

    let buyers_len = buyers.len();

    let strategy_profits: Vec<HashMap<Vec<i32>, u32>> = buyers
        .clone()
        .into_par_iter()
        .enumerate()
        .inspect(|(i, _)| println!("processing buyer {}/{}", i, buyers_len))
        .map(|(_, b)| b.apply_strategies(&strategies))
        .collect();

    let mut result = HashMap::new();

    for map in strategy_profits {
        for (key, value) in map {
            *result.entry(key).or_insert(0) += value;
        }
    }

    if let Some((strategy, profit)) = result.iter().max_by_key(|&(_, v)| v) {
        println!("Strategy {:?} is best and results in '{}'", strategy, profit);

        return *profit;
    }

    0
}

impl Solution for Day22 {
    fn solve_part1(&self, input: &str) -> String {
        let buyers = parse_input(input);
        buyers.iter()
            .map(|buyer| *buyer.evolve().last().unwrap())
            .sum::<u64>()
            .to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        buy_strategy(parse_input(input)).to_string()
    }
}
