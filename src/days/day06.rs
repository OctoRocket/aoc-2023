use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum SixthError {
    #[error("no times provided")]
    Time,

    #[error("no records provided")]
    Record,

    #[error("no distances recorded")]
    Distance
}

struct Race {
    time: usize,
    record: usize,
}

type Event = Vec<Race>;

fn parse_times(input: &str) -> Result<Event> {
    let mut event = vec![];
    let mut lines = input.split('\n');

    let times = lines
        .next()
        .ok_or(SixthError::Time)?
        .split(':')
        .nth(1)
        .ok_or(SixthError::Time)?
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    let records = lines
    .next()
        .ok_or(SixthError::Time)?
        .split(':')
        .nth(1)
        .ok_or(SixthError::Time)?
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    for index in 0..times.len() {
        event.push(Race {
            time: times[index],
            record: records[index],
        });
    }

    Ok(event)
}

pub fn first(input: &str) -> Result<usize> {
    let event = parse_times(input)?;

    let mut total_ways = vec![];

    for race in event {
        let mut ways = 0;

        for t in 0..=race.time {
            let distance = (race.time - t) * t;

            if distance > race.record {
                ways += 1;
            }
        }

        total_ways.push(ways);
    }

    dbg!(&total_ways);
    Ok(total_ways.into_iter().reduce(|a, b| a * b).ok_or(SixthError::Distance)?)
}
