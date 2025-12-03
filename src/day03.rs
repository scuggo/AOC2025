use color_eyre::{Report, eyre::ContextCompat};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

const BATTERIES: usize = 12;

pub struct BatteryBank {
    biggest_sorted: Vec<u8>,
    batteries: Vec<u8>,
}

impl TryFrom<String> for BatteryBank {
    type Error = Report;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let digits = value
            .chars()
            .map(|v| v.to_string().parse::<u8>())
            .collect::<Result<Vec<u8>, std::num::ParseIntError>>()?;
        let mut sorted_list = digits.clone();
        sorted_list.sort();
        sorted_list.reverse();
        sorted_list.dedup();

        Ok(Self {
            batteries: digits,
            biggest_sorted: sorted_list,
        })
    }
}

impl BatteryBank {
    fn try_value_part1(&self, index: usize) -> Result<Option<u64>, Report> {
        let value = self.biggest_sorted.get(index).context("out of bounds")?;
        let index = self.batteries.iter().position(|x| x == value).unwrap();
        let (_, split) = self.batteries.split_at(index + 1);
        if split.is_empty() {
            return Ok(None);
        }
        let mut split = split.to_vec();
        split.sort();
        let biggest = split.last().unwrap();

        Ok(Some((value * 10) as u64 + *biggest as u64))
    }
    fn part1(&self) -> Result<u64, Report> {
        let len = self.biggest_sorted.len();
        for i in 0..len {
            if let Some(v) = self.try_value_part1(i)? {
                return Ok(v);
            }
        }
        Ok(0)
    }

    fn part2(&self) -> Result<u64, Report> {
        let mut progress = vec![];
        let len = self.batteries.len();
        for (i, value) in self.batteries.iter().enumerate() {
            while let Some(last) = progress.last()
                && *last < value
                && progress.len() + (len - i) > BATTERIES
            {
                progress.pop();
            }

            if progress.len() < BATTERIES {
                progress.push(value);
            }
        }
        println!("value: {progress:?} {}", progress.len());
        let mut num = 0;
        for (i, v) in progress.iter().enumerate() {
            num += **v as u64 * 10_u64.pow(progress.len() as u32 - i as u32 - 1);
        }
        // println!("{num}");
        Ok(num)
    }
}

pub fn part1(data: &[BatteryBank]) -> Result<u64, Report> {
    Ok(data
        .iter()
        .map(|v| v.part1())
        .collect::<Result<Vec<u64>, Report>>()?
        .iter()
        .sum::<u64>())
}

pub fn part2(data: &[BatteryBank]) -> Result<u64, Report> {
    // let first = BatteryBank::try_from(String::from("818181911112111"))?.part2()?;
    Ok(data
        .iter()
        .map(|v| v.part2())
        .collect::<Result<Vec<u64>, Report>>()?
        .iter()
        .sum::<u64>())
}
pub fn data(filepath: &str) -> Result<Vec<BatteryBank>, Report> {
    let file = File::open(filepath)?;
    let bufreader = BufReader::new(file);
    let banks = bufreader
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()?
        .into_iter()
        .map(BatteryBank::try_from)
        .collect::<Result<Vec<BatteryBank>, Report>>()?;
    Ok(banks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::Criterion;
    use criterion_macro::criterion;

    #[criterion]
    fn bench_part1(b: &mut Criterion) {
        let data = data("inputs/day03.txt").unwrap();
        b.bench_function("day03-part1", |b| b.iter(|| part1(&data).unwrap()));
    }
    #[criterion]
    fn bench_part2(b: &mut Criterion) {
        let data = data("inputs/day03.txt").unwrap();
        b.bench_function("day03-part2", |b| b.iter(|| part2(&data).unwrap()));
    }
}
