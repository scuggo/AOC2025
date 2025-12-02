use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use color_eyre::{
    Report,
    eyre::{ContextCompat, eyre},
};

#[derive(Debug, Clone)]
enum DialDir {
    L(u32),
    R(u32),
}
impl From<DialDir> for i32 {
    fn from(val: DialDir) -> Self {
        match val {
            DialDir::L(v) => -(v as i32),
            DialDir::R(v) => v as i32,
        }
    }
}

impl TryFrom<&str> for DialDir {
    type Error = Report;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let prefix = value.get(0..1).context("Failed to get first char")?;
        match prefix {
            "L" => value[1..].parse().map(DialDir::L).map_err(|v| v.into()),
            "R" => value[1..].parse().map(DialDir::R).map_err(|v| v.into()),
            _ => Err(eyre!("Invalid Start")),
        }
    }
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    // let filepath = "testinput.txt";
    let filepath = "inputs/day1.txt";
    let file = File::open(filepath)?;
    let bufreader = BufReader::new(file);
    let mut lines = bufreader.lines();

    let mut total_value = 50;
    let mut code = 0;
    let mut code2 = 0;
    while let Some(Ok(line)) = lines.next() {
        let prev = total_value;
        let dialcode = DialDir::try_from(line.as_str())?;
        total_value += std::convert::Into::<i32>::into(dialcode.clone());
        let big_jump = total_value.div_euclid(100);
        total_value = total_value.rem_euclid(100);
        if big_jump < 0 && prev == 0 {
            code2 -= 1;
        }
        if big_jump > 0 && total_value == 0 {
            code2 -= 1;
        }
        code2 += big_jump.abs();
        if total_value == 0 {
            code += 1;
        }
        println!("{dialcode:?} {prev}->{total_value} {code} {code2}");
    }

    println!("code is {} {}!", code, code + code2);
    Ok(())
}
