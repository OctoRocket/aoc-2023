use regex::Regex;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum SecondError {
    #[error("possible empty line")]
    ParseGame,

    #[error("missing game tag")]
    GameTag,

    #[error("missing block type count")]
    BlockTypeCount,

    #[error("missing color type")]
    ColorType,

    #[error("missing color type {0}")]
    InvalidColorType(String),
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_to_set(input: &str) -> Result<Set> {
    let seperator = Regex::new(r"\d+ [^,]+")?;

    let blocks = seperator
        .find_iter(input)
        .map(|s| s.as_str().to_string())
        .collect::<Vec<String>>();

    let mut rgb = (0, 0, 0);

    for block in blocks {
        let count = block
            .split_whitespace()
            .next()
            .ok_or(SecondError::BlockTypeCount)?
            .parse()?;
        let color = block
            .split_whitespace()
            .nth(1)
            .ok_or(SecondError::ColorType)?;

        match color {
            "red" => {rgb.0 = count; Ok(())},
            "green" => {rgb.1 = count; Ok(())},
            "blue" => {rgb.2 = count; Ok(())},
            e => Err(SecondError::InvalidColorType(e.to_string()))
        }?;
    }

    Ok(Set {
        red: rgb.0,
        green: rgb.1,
        blue: rgb.2,
    })
}

fn parse_to_game(input: &str) -> Result<Game> {
    let full_scan = Regex::new(r"Game \d+|\d+ \w+")?;
    let id_scan = Regex::new(r"\d+")?;
    let set_scan = Regex::new(r"\d+ [^;]+")?;

    let game_id = id_scan
        .find(full_scan.find_iter(input)
            .next()
            .ok_or(SecondError::ParseGame)?
            .as_str())
        .ok_or(SecondError::GameTag)?
        .as_str()
        .parse()?;
    
    let mut sets = Vec::new();
    for set in set_scan.find_iter(input)
        .map(|s| parse_to_set(s.as_str())) {
        sets.push(set?);
    }

    Ok(Game{id: game_id, sets})
}

fn possible_games(input: Vec<Game>, available_colors: (u32, u32, u32)) -> Vec<Game> {
    let mut possible = Vec::new();

    for game in input {
        let mut success = true;

        for set in &game.sets {
            let results = (
                set.red <= available_colors.0,
                set.green <= available_colors.1,
                set.blue <= available_colors.2,
            );

            if !results.0 || !results.1 || !results.2 {
                success = false;
            }
        }

        if success {
            possible.push(game);
        }
    }

    possible
}

pub fn first(input: &str, available_colors: (u32, u32, u32)) -> Result<u32> {
    let mut sum = 0;
    let mut games = Vec::new();

    for line in input.split('\n') {
        games.push(parse_to_game(line)?);
    }

    let validated_games = possible_games(games, available_colors);

    for game in validated_games {
        sum += game.id;
    }

    Ok(sum)
}

fn minimum_colors(game: Game) -> (u32, u32, u32) {
    let mut maximums = (0, 0, 0);

    for set in game.sets {
        if set.red   > maximums.0 { maximums.0 = set.red;   }
        if set.green > maximums.1 { maximums.1 = set.green; }
        if set.blue  > maximums.2 { maximums.2 = set.blue;  }
    }

    maximums
}

pub fn second(input: &str) -> Result<u32> {
    let mut games = Vec::new();
    for line in input.split('\n') {
        games.push(parse_to_game(line)?);
    }

    let mut powers = Vec::new();
    for game in games {
        let minimums = minimum_colors(game);
        powers.push(minimums.0 * minimums.1 * minimums.2);
    }

    Ok(powers.iter().sum())
}