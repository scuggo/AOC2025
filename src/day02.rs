use color_eyre::{Report, eyre::ContextCompat};
use std::{
    fs::File,
    io::Read,
    ops::{Div, Rem},
};

#[derive(Debug)]
pub struct IdRange {
    start: u64,
    end: u64,
}

impl TryFrom<&str> for IdRange {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split("-");
        let start: u64 = split.next().context("Invalid start ID")?.parse()?;
        let end: u64 = split.next().context("Invalid end ID")?.parse()?;
        Ok(Self { start, end })
    }
}

impl IdRange {
    fn get_invalid(&self) -> Result<Vec<u64>, Report> {
        Ok((self.start..=self.end)
            .filter(|v| {
                let len = v.checked_ilog10().unwrap_or(0) + 1;
                if !len.is_multiple_of(2) {
                    return false;
                }
                let d = 10_u64.pow(len / 2);
                let first = v.div(d);
                let second = v.rem(d);
                first == second
            })
            .collect())
    }

    fn get_invalid2(&self) -> Result<Vec<u64>, Report> {
        Ok((self.start..=self.end)
            .filter(|v| {
                let len = (v.checked_ilog10().unwrap_or(0) + 1) as usize;
                let mut digits = vec![0; len];
                let mut temp = *v;
                for i in 1..=len {
                    digits[len - i] = temp % 10;
                    temp /= 10;
                }
                (1..=len)
                    .filter(|v| len.is_multiple_of(*v) && (len / v) != 1)
                    .any(|c| {
                        let mut chunks = digits.chunks(c);
                        chunks
                            .next()
                            .is_some_and(|first| chunks.all(|chunk| chunk == first))
                    })
            })
            .collect())
    }
}

pub fn part1(data: &[IdRange]) -> Result<u64, Report> {
    Ok(data
        .iter()
        .map(|v| v.get_invalid().map(|v| v.into_iter().sum::<u64>()))
        .collect::<Result<Vec<u64>, Report>>()?
        .into_iter()
        .sum::<u64>())
}

pub fn part2(data: &[IdRange]) -> Result<u64, Report> {
    Ok(data
        .iter()
        .map(|v| v.get_invalid2().map(|v| v.into_iter().sum::<u64>()))
        .collect::<Result<Vec<u64>, Report>>()?
        .into_iter()
        .sum::<u64>())
}
pub fn data(filepath: &str) -> Result<Vec<IdRange>, Report> {
    let mut file = File::open(filepath)?;
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
        .trim_end()
        .split(",")
        .map(IdRange::try_from)
        .collect::<Result<Vec<IdRange>, Report>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::Criterion;
    use criterion_macro::criterion;

    #[criterion]
    fn bench_part1(b: &mut Criterion) {
        let data = data("inputs/day02.txt").unwrap();
        b.bench_function("day02-part1", |b| b.iter(|| part1(&data).unwrap()));
    }
    #[criterion]
    fn bench_part2(b: &mut Criterion) {
        let data = data("inputs/day01.txt").unwrap();
        b.bench_function("day02-part2", |b| b.iter(|| part2(&data).unwrap()));
    }
}
