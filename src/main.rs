use std::{fs::File, io::Read};
use anyhow::Result;

mod first {
    fn parse_text_numbers(input: String) -> String {
        input
            .replace("one",   "1")
            .replace("two",   "2")
            .replace("three", "3")
            .replace("four",  "4")
            .replace("eight", "8")
            .replace("five",  "5")
            .replace("six",   "6")
            .replace("seven", "7")
            .replace("nine",  "9")
    }

    pub fn second(input: String) -> String {
        parse_text_numbers(input)
    }
}

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("{}", first::second(buf));

    Ok(())
}
