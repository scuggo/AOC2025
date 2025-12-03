use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines, Read},
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

pub fn part1(lines: &[String]) -> Result<i32, Report> {
    let mut total_value = 50;
    let mut code = 0;
    for line in lines {
        let dialcode = DialDir::try_from(line.as_str())?;
        total_value += std::convert::Into::<i32>::into(dialcode.clone());
        total_value = total_value.rem_euclid(100);
        if total_value == 0 {
            code += 1;
        }
    }
    Ok(code)
}

pub fn part2(lines: &[String]) -> Result<i32, Report> {
    let mut total_value = 50;
    let mut code = 0;
    for line in lines {
        let prev = total_value;
        let dialcode = DialDir::try_from(line.as_str())?;
        total_value += std::convert::Into::<i32>::into(dialcode.clone());
        let big_jump = total_value.div_euclid(100);
        total_value = total_value.rem_euclid(100);
        if big_jump < 0 && prev == 0 {
            code -= 1;
        }
        if big_jump > 0 && total_value == 0 {
            code -= 1;
        }
        code += big_jump.abs();
        if total_value == 0 {
            code += 1;
        }
    }
    Ok(code)
}

pub fn data(filepath: &str) -> Result<Vec<String>, Report> {
    let file = File::open(filepath)?;
    let bufreader = BufReader::new(file);
    let lines = bufreader
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()?;
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::Criterion;
    use criterion_macro::criterion;

    #[criterion]
    fn bench_part1(b: &mut Criterion) {
        let data = data("inputs/day01.txt").unwrap();
        b.bench_function("day01-part1", |b| b.iter(|| part1(&data).unwrap()));
    }
    #[criterion]
    fn bench_part2(b: &mut Criterion) {
        let data = data("inputs/day01.txt").unwrap();
        b.bench_function("day01-part2", |b| b.iter(|| part2(&data).unwrap()));
    }
}
