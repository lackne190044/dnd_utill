use std::cmp::max;
use std::io;
use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Roll {
    sides: u8,
    num_dice: u8,
    modifier: i16,
}

impl Roll {
    pub fn roll(&mut self) {
        let mut sum: u64 = 0;
        println!("-----------------------------------------");
        for _ in 0..self.num_dice {
            let roll: i16 = i16::from(rand::random_range(1..=self.sides));
            println!("{} ", roll);
            let to_add = max(roll + self.modifier, 0);
            sum += to_add as u64;
        }
        println!("-----------------------------------------");
        println!("Sum: {}", sum);
    }
}

fn main() {
    loop {
        let input = get_input().unwrap();
        if input == "q" { return; }
        let mut roll = match parse(&input) {
            Ok(roll) => roll,
            Err(e) => {
                println!("Error: {}", e);
                continue
            }
        };
        (&mut roll).roll();
    }
}

fn get_input() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn parse(input: &str) -> Result<Roll> {
    let mut tmp = input.split("d").collect::<Vec<&str>>();
    let sides: &str;
    let modifier: String;
    if tmp.len() != 2 { return Err(anyhow!("Invalid input format should be 'XdY+m'")) };
    let num_dice = tmp[0];
    let second_half = tmp[1].to_string();
    if second_half.contains("+") {
        tmp = second_half.split("+").collect::<Vec<&str>>();
        if tmp.len() != 2 { return Err(anyhow!("Invalid input format should be 'XdY+m'")) };
        sides = tmp[0];
        modifier = tmp[1].to_string();
    } else if second_half.contains("-") {
        tmp = second_half.split("-").collect::<Vec<&str>>();
        if tmp.len() != 2 { return Err(anyhow!("Invalid input format should be 'XdY+m'")) };
        sides = tmp[0];
        modifier = format!("-{}", tmp[1]);
    } else {
        sides = &second_half;
        modifier = "0".to_string();
    }

    Ok(Roll {
        sides: sides.parse::<u8>()?,
        num_dice: num_dice.parse::<u8>()?,
        modifier: modifier.parse::<i16>()?,
        })
}
