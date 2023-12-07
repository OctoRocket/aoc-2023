#![allow(dead_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod days;

use anyhow::Result;
use std::{
    fs::File,
    io::Read,
};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("{:?}", days::day06::first(&buf)?);

    Ok(())
}