use color_eyre::{Report, eyre::ContextCompat};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

struct Wall {
    inner: Vec<bool>,
    width: usize,
    height: usize,
}

impl<R: BufRead> TryFrom<Lines<R>> for Wall {
    type Error = Report;
    fn try_from(mut value: Lines<R>) -> Result<Self, Self::Error> {
        let width = None;
        while let Some(line) = value.next() {
            let line = line?;
            let chars: Vec<char> = line.chars().collect();
            let new_width = chars.len();
            if let Some(old_width) = width {
                return Err();
            }
            let mut inner: Vec<bool> = Vec::with_capacity(width);
            for c in chars {
                match c {
                    '#' => inner.push(true),
                    '.' => inner.push(false),
                    _ => return Err(Report::msg(format!("Invalid character in wall: {}", c))),
                }
            }
            let height = 1;
        }
        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

pub fn part1(data: &Wall) -> Result<u64, Report> {
    // Ok(data
    //     .iter()
    //     .map(|v| v.part1())
    //     .collect::<Result<Vec<u64>, Report>>()?
    //     .iter()
    //     .sum::<u64>())
    Ok(0)
}

pub fn part2(data: &Wall) -> Result<u64, Report> {
    // let first = BatteryBank::try_from(String::from("818181911112111"))?.part2()?;
    // Ok(data
    //     .iter()
    //     .map(|v| v.part2())
    //     .collect::<Result<Vec<u64>, Report>>()?
    //     .iter()
    //     .sum::<u64>())
    Ok(0)
}
pub fn data(filepath: &str) -> Result<Wall, Report> {
    let file = File::open(filepath)?;
    let bufreader = BufReader::new(file);
    let wall = Wall::try_from(bufreader.lines())?;
    Ok(wall)
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::Criterion;
    use criterion_macro::criterion;

    #[criterion]
    fn bench_part1(b: &mut Criterion) {
        let data = data("inputs/day04.txt").unwrap();
        b.bench_function("day04-part1", |b| b.iter(|| part1(&data).unwrap()));
    }
    #[criterion]
    fn bench_part2(b: &mut Criterion) {
        let data = data("inputs/day04.txt").unwrap();
        b.bench_function("day04-part2", |b| b.iter(|| part2(&data).unwrap()));
    }
}
