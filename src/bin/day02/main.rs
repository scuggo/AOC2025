use color_eyre::{Report, eyre::ContextCompat};
use std::{fs::File, io::Read};

#[derive(Debug)]
struct IdRange {
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
        Ok((self.start..self.end + 1)
            .filter(|v| {
                let str = v.to_string();
                let len = str.len();
                if (len % 2) != 0 {
                    return false;
                }
                str[0..len / 2] == str[len / 2..len]
            })
            .collect())
    }
    fn get_invalid2(&self) -> Result<Vec<u64>, Report> {
        Ok((self.start..self.end + 1)
            .filter(|v| {
                let str = v.to_string();
                let len = str.len();
                (1..len + 1)
                    .filter(|v| (len % v) == 0)
                    .filter(|v| (len / v) != 1)
                    .filter(|v| {
                        let strchunk = str.chars().collect::<Vec<char>>();
                        let strchunk = strchunk.chunks(*v).collect::<Vec<&[char]>>();
                        !strchunk.iter().any(|v| strchunk.iter().any(|x| v != x))
                    })
                    .sum::<usize>()
                    > 0
            })
            .collect())
    }
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    // let filepath = "testinput2.txt";
    let filepath = "inputs/day2.txt";
    let mut file = File::open(filepath)?;
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    let ranges = string
        .trim_end()
        .split(",")
        .map(IdRange::try_from)
        .collect::<Result<Vec<IdRange>, Report>>()?;

    let code1 = ranges
        .iter()
        .map(|v| v.get_invalid().map(|v| v.into_iter().sum::<u64>()))
        .collect::<Result<Vec<u64>, Report>>()?
        .into_iter()
        .sum::<u64>();
    println!("code1: {code1}");
    let code2 = ranges
        .iter()
        .map(|v| v.get_invalid2().map(|v| v.into_iter().sum::<u64>()))
        .collect::<Result<Vec<u64>, Report>>()?
        .into_iter()
        .sum::<u64>();
    println!("code1: {code2}");
    Ok(())
}
