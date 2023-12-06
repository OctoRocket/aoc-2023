use std::num::ParseIntError;

use anyhow::Result;
use thiserror::Error;


#[derive(Debug, Error)]
enum FifthError {
    #[error("missing content")]
    Content,

    #[error("missing seeds")]
    Seed,

    #[error("missing map data")]
    MapData,

    #[error("no answers")]
    Answer,

    #[error("line of map is incomplete, line: {0}")]
    Line(String),

    #[error("no answers for pair: {0}")]
    Pair(String),
}

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<[usize; 3]>>,
}

#[allow(clippy::needless_range_loop)]
fn parse_input(input: &str) -> Result<Almanac> {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let mut seeds = vec![];
    for seed in sections
        .first()
        .ok_or(FifthError::Content)?
        .split(':')
        .nth(1)
        .ok_or(FifthError::Seed)?
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect::<Vec<Result<usize, ParseIntError>>>() {
        seeds.push(seed?);
    }

    let mut maps = vec![];
    for section in &sections[1..sections.len()] {
        let mut map = vec![];

        for parameters in section
                .split(':')
                .nth(1)
                .ok_or(FifthError::MapData)?
                .split('\n') {
            if !parameters.is_empty() {
                let mut line = parameters.split_whitespace().map(|s| s.parse().unwrap());
                let mut line_map = [0; 3];

                for i in 0..=2 {
                    line_map[i] = line.next().ok_or(FifthError::Line(parameters.to_string()))?;
                }

                map.push(line_map);
            }
        }
        maps.push(map);
    }

    Ok(Almanac { seeds, maps })
}

fn map_seed(seed: usize, map: &Vec<[usize; 3]>) -> usize {
    for submap in map {
        if (submap[1]..(submap[1] + submap[2])).contains(&seed) {
            return seed - submap[1] + submap[0];
        }
    }

    seed
}

pub fn first(input: &str) -> Result<usize> {
    let almanac = parse_input(input)?;
    println!("Almanac generated.");

    let mut answers = vec![];
    for mut seed in almanac.seeds {
        for map in &almanac.maps {
            seed = map_seed(seed, map);
        }
        println!("{seed}");
        answers.push(seed);
    }

    Ok(*answers.iter().min().ok_or(FifthError::Answer)?)
}

pub fn second(input: &str) -> Result<usize> {
    let almanac = parse_input(input)?;
    println!("Almanac generated.");

    let seed_pairs = almanac.seeds.chunks_exact(2).collect::<Vec<&[usize]>>();

    let mut answers = vec![];
    for seed_pair in seed_pairs {
        let mut pair_answers = vec![];
        for mut seed in seed_pair[0]..(seed_pair[0] + seed_pair[1]) {
            for map in &almanac.maps {
                seed = map_seed(seed, map);
            }
            pair_answers.push(seed);
        }
        answers.push(*pair_answers.iter().min().ok_or(FifthError::Pair(format!("{:?}", &seed_pair)))?);
        println!("Pair complete");
    }

    Ok(*answers.iter().min().ok_or(FifthError::Answer)?)
}
