use anyhow::Result;
use thiserror::Error;
use std::cmp::Ordering;

#[derive(Debug, Error)]
enum SeventhError {
    #[error("line doesn't have hand")]
    Hand,

    #[error("line doesn't have bet")]
    Bet,

    #[error("couldn't parse card {0} to value")]
    Card(String),

    #[error("hand is empty")]
    Empty,
}

#[derive(Debug, Eq, Clone)]
struct Play {
    hand: String,
    bet: usize,
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = rank_hand(self)
            .unwrap_or((0, vec![0, 0, 0, 0, 0]));

        let other_value = rank_hand(other)
            .unwrap_or((0, vec![0, 0, 0, 0, 0]));

        if self_value.0.eq(&other_value.0) {
            for index in 0..self_value.1.len() {
                if !self_value.1[index].eq(&other_value.1[index]) {
                    return self_value.1[index].cmp(&other_value.1[index]);
                }
            }

            return Ordering::Equal;
        }

        self_value.0.cmp(&other_value.0)
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        let self_value = rank_hand(self)
        .unwrap_or((0, vec![0, 0, 0, 0, 0]));
        if self_value == (0, vec![0, 0, 0, 0, 0]) {
            println!("WARNING, invalid hand {:?}", &self);
        }

        let other_value = rank_hand(other)
        .unwrap_or((0, vec![0, 0, 0, 0, 0]));
        if other_value == (0, vec![0, 0, 0, 0, 0]) {
            println!("WARNING, invalid hand {:?}", &other);
        }

        self_value == other_value
    }
}

type Game = Vec<Play>;

fn get_card_number(card: char) -> Result<usize> {
    match card {
        'A' => Ok(12), 
        'K' => Ok(11), 
        'Q' => Ok(10), 
        'T' => Ok( 9), 
        '9' => Ok( 8), 
        '8' => Ok( 7), 
        '7' => Ok( 6), 
        '6' => Ok( 5), 
        '5' => Ok( 4), 
        '4' => Ok( 3), 
        '3' => Ok( 2), 
        '2' => Ok( 1),
        'J' => Ok( 0),
        _ => Err(SeventhError::Card(card.to_string()).into())
    }
}

/*
    6: Five of a kind, where all five cards have the same label: AAAAA
    5: Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    4: Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    3: Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    2: Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    1: One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    0: High card, where all cards' labels are distinct: 23456
*/
fn determine_hand(mut cards: Vec<usize>) -> Result<usize> {
    let mut max_card = cards.iter().max().ok_or(SeventhError::Empty)?.to_owned();

    if max_card == cards[0] {
        let jokers = cards[0]; // if jokers are the max card
        cards[0] = 0;
        max_card = cards.iter().max().ok_or(SeventhError::Empty)?.to_owned();

        let index_of_max = cards.iter().position(|n| n == &max_card).unwrap();
        cards[index_of_max] += jokers;
        max_card = cards.iter().max().ok_or(SeventhError::Empty)?.to_owned();
    } else {
        for _ in 0..cards[0] { // for each joker
            cards[0] -= 1;
            let index_of_max = cards.iter().position(|n| n == &max_card).unwrap();
            cards[index_of_max] += 1;
            max_card += 1;
        }
    }

    if max_card == 5 {
        return Ok(6);
    } else if max_card == 4  {
        return Ok(5);
    } else if max_card == 3 && cards.contains(&2) {
        return Ok(4);
    } else if max_card == 3 {
        return Ok(3);
    } else if cards.iter().filter(|v| v == &&2).count() == 2 {
        return Ok(2);
    } else if max_card == 2 {
        return Ok(1);
    } else if max_card > 5 {
        panic!("Error, over 5 cards accounted for with cards {:?} with {} jokers.", cards, cards[0]);
    }

    Ok(0)
}

fn rank_hand(input: &Play) -> Result<(usize, Vec<usize>)> {
    let hand = &input.hand;
    let mut cards = vec![0; 13];
    let mut card_numbers = vec![];

    for card in hand.chars() {
        let card_number = get_card_number(card)?;
        cards[card_number] += 1;
        card_numbers.push(card_number);
    }

    let hand_value = determine_hand(cards)?;
    
    Ok((hand_value, card_numbers))
}

fn parse(input: &str) -> Result<Game> {
    let mut game = Game::new();

    for line in input.split('\n') {
        let mut split = line.split_whitespace();
        let hand = split.next().ok_or(SeventhError::Hand)?.to_string();
        let bet = split.next().ok_or(SeventhError::Bet)?.parse()?;

        game.push(Play { hand, bet });
    }

    Ok(game)
}

pub fn second(input: &str) -> Result<usize> {
    let mut game = parse(input)?;
    game.sort_unstable();
    // game.iter().enumerate().for_each(|h| println!("{} => {:?}", h.1.hand, rank_hand(h.1)));

    let mut sum = 0;
    for rank in 1..=game.len() {
        sum += game[rank - 1].bet * rank;
    }

    Ok(sum)
}
