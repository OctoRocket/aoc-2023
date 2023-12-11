#![allow(dead_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod days;

use anyhow::Result;
use thiserror::Error;
use std::{
    fs::File,
    io::Read,
    env::args,
};

#[derive(Debug, Error)]
enum RunnerError {
    #[error("missing file name")]
    File,
}

fn main() -> Result<()> {
    let mut file = File::open(args().nth(1).ok_or(RunnerError::File)?)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("{:?}", days::day07::second(&buf)?);

    Ok(())
}