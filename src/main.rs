#![allow(dead_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
use std::{
    fs::File,
    io::Read,
};
use anyhow::Result;

mod second;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("{:?}", second::second(&buf)?);

    Ok(())
}
