use std::{collections::HashMap, num::ParseIntError};

use anyhow::Result;
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

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<HashMap<usize, usize>>,
}

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
            if !parameters.is_empty() {
                let map_results: Vec<Result<usize, ParseIntError>> = parameters.split_whitespace().map(str::parse).collect();
                let mut map_vals = vec![];

                for result in map_results {
                    map_vals.push(result?);
                }

                for i in 0..map_vals[2] {
                    map.insert(map_vals[0] + i, map_vals[1] + i);
                }
            }
        }

        maps.push(map);
    }

    Ok(Almanac { seeds, maps })
}

pub fn first(input: &str) -> Result<usize> {
    let almanac = parse_input(input)?;

    let mut answers = vec![];
    for mut seed in almanac.seeds {
        for map in &almanac.maps {
            seed = *map.get(&seed).ok_or(FifthError::Content)?;
        }
        answers.push(seed);
    }

    Ok(*answers.iter().min().ok_or(FifthError::Content)?)
}
