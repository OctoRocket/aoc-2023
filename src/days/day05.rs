use std::{collections::HashMap, num::ParseIntError};

use anyhow::Result;
use aoc_runner_derive::{
    aoc,
    aoc_generator,
};
use thiserror::Error;

#[derive(Debug, Error)]
enum FifthError {
    #[error("missing content")]
    Content,

    #[error("missing seeds")]
    Seed,

    #[error("missing map data")]
    MapData
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<HashMap<usize, usize>>,
}

#[aoc_generator(day5)]
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
        let mut map = HashMap::new();

        for parameters in section
            .split(':')
            .nth(1)
            .ok_or(FifthError::MapData)?
            .split('\n') {
            let map_results: Vec<Result<usize, ParseIntError>> = parameters.split_whitespace().map(str::parse).collect();
            let mut map_vals = vec![];

            for result in map_results {
                map_vals.push(result?)
            }

            for i in 0..map_vals[2] {
                map.insert(map_vals[0] + i, map_vals[1] + i);
            }
        }

        maps.push(map);
    }

    Ok(Almanac { seeds, maps })
}

#[aoc(day5, part1)]
pub fn first(input: Result<Almanac>) -> Almanac {
    input.unwrap()
}