use color_eyre::{
    Report,
    eyre::{ContextCompat, eyre},
};
use std::{
    fmt::Display,
    fs::File,
    io::Read,
    ops::{Div, Rem},
};

#[derive(Debug, Clone)]
pub enum BoardBit {
    Source,
    Split,
    Beam,
    Empty,
}

impl From<char> for BoardBit {
    fn from(value: char) -> Self {
        match value {
            '.' => BoardBit::Empty,
            'S' => BoardBit::Source,
            '|' | '-' => BoardBit::Beam,
            '^' => BoardBit::Split,
            _ => BoardBit::Empty,
        }
    }
}

impl Display for BoardBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            BoardBit::Empty => '.',
            BoardBit::Source => 'S',
            BoardBit::Beam => '|',
            BoardBit::Split => '^',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
pub struct Board {
    data: Vec<BoardBit>,
    width: usize,
    height: usize,
}

impl TryFrom<&str> for Board {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let mut width = None;
        let mut height = 0;
        let mut data = Vec::new();
        for line in lines.by_ref() {
            height += 1;
            if width.is_none() {
                width = Some(line.len());
            } else if width.unwrap() != line.len() {
                return Err(eyre!("Inconsistent line lengths in board"));
            }
            for c in line.chars() {
                let bit = BoardBit::from(c);
                data.push(bit);
            }
        }
        Ok(Self {
            height,
            width: width.unwrap_or(0),
            data,
        })
    }
}

impl Board {
    fn get_row(&self, row: usize) -> Option<&[BoardBit]> {
        if row >= self.height {
            return None;
        }
        let start = row * self.width;
        let end = start + self.width;
        Some(&self.data[start..end])
    }
}

pub fn part1(data: &Board) -> Result<u64, Report> {
    let mut splits = 0;
    let mut cur = data.get_row(0).context("Invalid row")?.to_vec();
    let mut newdata = Vec::new();
    newdata.extend(cur.clone());
    for row in 1..data.height {
        let line = data.get_row(row).context("Invalid row")?;
        println!("Row {row}: {:?}", line);
        let mut new_row = vec![BoardBit::Empty; data.width];
        for col in 0..data.width {
            let new = &new_row[col];
            let prev = &cur[col];
            let next = &line[col];
            match (prev, next, new) {
                (BoardBit::Source, BoardBit::Empty, _) => {
                    new_row[col] = BoardBit::Beam;
                }
                (BoardBit::Beam, BoardBit::Empty, _) => {
                    new_row[col] = BoardBit::Beam;
                }
                (BoardBit::Beam, BoardBit::Split, _) => {
                    splits += 1;
                    new_row[col] = BoardBit::Split;
                    if col > 0 {
                        new_row[col - 1] = BoardBit::Beam;
                    }
                    if col < data.width - 1 {
                        new_row[col + 1] = BoardBit::Beam;
                    }
                }
                _ => {}
            }
        }
        println!(
            "cur : {:?}",
            cur.iter().map(|v| v.to_string()).collect::<String>()
        );
        println!(
            "next: {:?}",
            line.iter().map(|v| v.to_string()).collect::<String>()
        );
        println!(
            "new : {:?}",
            new_row.iter().map(|v| v.to_string()).collect::<String>()
        );
        newdata.extend(new_row.clone());
        cur = new_row;
    }
    // pretty print
    for row in 0..data.height {
        let line = newdata.chunks(data.width).nth(row).context("Invalid row")?;
        let line_str: String = line.iter().map(|v| v.to_string()).collect();
        println!("{}", line_str);
    }
    Ok(splits)
}

pub fn part2(data: &Board) -> Result<u64, Report> {
    Ok(0)
}
pub fn data(filepath: &str) -> Result<Board, Report> {
    let mut file = File::open(filepath)?;
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    let board = Board::try_from(string.as_str())?;
    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::Criterion;
    use criterion_macro::criterion;

    #[criterion]
    fn bench_part1(b: &mut Criterion) {
        let data = data("inputs/day07.txt").unwrap();
        b.bench_function("day07-part1", |b| b.iter(|| part1(&data).unwrap()));
    }
    #[criterion]
    fn bench_part2(b: &mut Criterion) {
        let data = data("inputs/day07.txt").unwrap();
        b.bench_function("day07-part2", |b| b.iter(|| part2(&data).unwrap()));
    }
}
