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

fn grab_cards_by_id(id: usize, cards: Vec<Card>) -> Option<Card> {
    cards.into_iter().find(|i| i.id == id)
}

pub fn second(input: &str) -> Result<u32> {
    let mut cards = generate_cards(input)?;

    let mut total_cards = 0;
    let mut cards_left = cards.len();
    let mut prev_id = 0;
    while cards_left > 0 {
        let card = &cards[0];
        let score = card.score;
        let current_id = card.id;

        if current_id != prev_id {
            println!("{current_id}");
            prev_id = current_id;
        }

        let mut copied_cards = vec![];
        for id in 1..=score {
            if let Some(copy) = grab_cards_by_id(current_id + id, cards.clone()) {
                copied_cards.push(copy);
            }
        }

        cards.remove(0);
        cards_left -= 1;

        copied_cards.append(&mut cards);
        cards = copied_cards;
        cards_left += score;

        total_cards += 1;

        sort_by_ids(&mut cards);
    }

    Ok(total_cards)
}
