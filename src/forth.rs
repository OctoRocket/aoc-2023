use std::vec;

use anyhow::Result;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
enum ForthError {
    #[error("ID missing, possible missing line: \n{0}")]
    IdError(String),

    #[error("Numbers missing in line: \n{0}")]
    NumberError(String),
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    score: usize,
}

fn generate_cards(input: &str) -> Result<Vec<Card>> {
    let card_scan = Regex::new(r"\d+")?;
    let number_scan = Regex::new(r"[\d\s]+\|[\d\s]+")?;
    let split_scan = Regex::new(r"[^|]+")?;
    let value_scan = Regex::new(r"\d+")?;

    let mut cards = vec![];

    for line in input.split('\n') {
        let id = card_scan
            .find(line)
            .ok_or(ForthError::IdError(line.to_string()))?
            .as_str()
            .parse()?;

        let numbers = number_scan
            .find(line)
            .ok_or(ForthError::NumberError(line.to_string()))?
            .as_str();

        let winning_values = value_scan.find_iter(
            split_scan
                .find_iter(numbers)
                .next()
                .ok_or(ForthError::NumberError(line.to_string()))?
                .as_str()
            )
            .map(|n| n.as_str().parse().unwrap())
            .collect::<Vec<u32>>();

        let owned_values = value_scan.find_iter(
            split_scan
                .find_iter(numbers)
                .nth(1)
                .ok_or(ForthError::NumberError(line.to_string()))?
                .as_str()
            )
            .map(|n| n.as_str().parse().unwrap())
            .collect::<Vec<u32>>();

        let score = owned_values
            .iter()
            .filter(|n| winning_values.contains(n))
            .collect::<Vec<&u32>>()
            .len();

        cards.push(
            Card {
                id,
                score,
            }
        );
    }

    Ok(cards)
}

#[allow(dead_code)]
fn print_ids(cards: &[Card]) {
    println!("{:?}", cards.iter().map(|c| c.id).collect::<Vec<usize>>());
}

fn sort_by_ids(cards: &mut [Card]) {
    cards.sort_unstable_by(|c1, c2| c1.id.cmp(&c2.id));
}

pub fn second(input: &str) -> Result<usize> {
    let card_scores = generate_cards(input)?.iter().map(|c| c.score).collect::<Vec<usize>>();
    let mut card_numbers = vec![1; card_scores.len()];
    card_numbers.fill(1);

    for index in 0..card_scores.len() {
        for i in 1..=card_scores[index] {
            if card_numbers.get(index + i).is_some() {
                card_numbers[index + i] += card_numbers[index];
            }
        }
    }

    Ok(card_numbers.into_iter().sum())
}
