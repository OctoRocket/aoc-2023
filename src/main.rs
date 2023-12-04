#![allow(dead_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::Read,
};
use anyhow::Result;

mod forth;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("{:?}", forth::first(&buf)?);

    Ok(())
}
