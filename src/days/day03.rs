use anyhow::Result;
use regex::Regex;

struct Number {
    value: u32,
    adjacent_to_symbol: bool,
    indices: Vec<(i32, i32)>,
}

fn parse_to_string(input: &str) -> Vec<String> {
    input.split('\n').map(String::from).collect::<Vec<String>>()
}

fn validate_index(index: &(i32, i32), height: i32, width: i32) -> bool {
    let mut valid = true;
    let (x, y) = index;

    if x < &0 || x > &width {
        valid = false;
    }
    if y < &0 || y > &height {
        valid = false;
    }

    valid
}

fn generate_adjacents(indices: Vec<(i32, i32)>, height: i32, width: i32) -> Vec<Vec<(i32, i32)>> {
    let mut adjacent_indices = Vec::new();

    for index in indices {
        let (x, y) = index;
        let adjacents = vec![
            (x - 1, y - 1),
            (x    , y - 1),
            (x + 1, y - 1),

            (x - 1, y),
            (x + 1, y),

            (x - 1, y + 1),
            (x    , y + 1),
            (x + 1, y + 1),
        ];

        adjacent_indices.push(
            adjacents
            .iter()
            .filter(|i| validate_index(i, height, width))
            .map(|i| i.to_owned())
            .collect::<Vec<(i32, i32)>>()
        );
    }

    adjacent_indices
}

fn get_numbers(data: Vec<String>) -> Result<Vec<Number>> {
    let number_scan = Regex::new(r"\d+")?;

    for line in data {
        
    }

    todo!()
}

pub fn first(input: &str) -> Result<u32> {
    let data = parse_to_string(input);
    let numbers = get_numbers(data)?;

    todo!()
}
