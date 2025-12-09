use color_eyre::{
    Report,
    eyre::{ContextCompat, eyre},
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};
#[derive(Debug)]
enum Symbol {
    Add,
    Mul,
}

#[derive(Debug)]
enum Either {
    Num(usize),
    Symbol(Symbol),
}

impl Either {
    fn is_symbol(&self) -> bool {
        matches!(self, Either::Symbol(_))
    }
    fn is_num(&self) -> bool {
        matches!(self, Either::Num(_))
    }
}

impl Symbol {
    fn apply(&self, data: &Vec<usize>) -> usize {
        let mut val = 0;
        for (i, v) in data.iter().enumerate() {
            match (self, i) {
                (Symbol::Add, _) => val += v,
                (Symbol::Mul, 0) => val = *v,
                (Symbol::Mul, _) => val *= v,
            }
        }
        val
    }
}

#[derive(Debug)]
pub struct MultGrid {
    data: Vec<Vec<usize>>,
    symbols: Vec<Symbol>,
    width: usize,
    num_height: usize,
}

impl<R: BufRead> TryFrom<Lines<R>> for MultGrid {
    type Error = Report;
    fn try_from(value: Lines<R>) -> Result<Self, Self::Error> {
        let lines = value.collect::<Result<Vec<String>, _>>()?;
        let mut data = Vec::new();
        let mut symbols = Vec::new();
        let mut num_height = 0;
        let mut widths = Vec::new();
        let parts: Vec<char> = lines.last().unwrap().chars().collect();
        let mut length = 0;
        for c in parts {
            if c.is_ascii_digit() || c.is_whitespace() {
                length += 1;
            } else {
                symbols.push(match c {
                    '+' => Symbol::Add,
                    '*' => Symbol::Mul,
                    v => {
                        panic!("unexpected symbol: {v:?}")
                    }
                });
                widths.push(length);
                length = 0;
            }
        }
        widths.remove(0);
        widths.push(length + 1);

        for line in lines.iter().rev().skip(1).rev() {
            let parts: Vec<char> = line.chars().collect();
            num_height += 1;
            let mut widths_index = 0;
            for w in &widths {
                let part = parts[widths_index..widths_index + *w]
                    .iter()
                    .map(|v| {
                        if v.is_ascii_digit() {
                            v.to_digit(10).unwrap() as usize
                        } else {
                            0
                        }
                    })
                    .collect::<Vec<usize>>();

                data.push(part);
                widths_index += w + 1;
            }
        }
        Ok(Self {
            width: widths.len(),
            num_height,
            data,
            symbols,
        })
    }
}

impl MultGrid {
    pub fn get_num_col(&self, index: usize) -> Vec<&Vec<usize>> {
        (0..self.num_height)
            .map(|x| (x * self.width) + index)
            .map(|i| &self.data[i])
            .collect()
    }
}

pub fn part1(data: &MultGrid) -> Result<u64, Report> {
    let mut total = 0;
    for i in 0..data.width {
        let col = data
            .get_num_col(i)
            .into_iter()
            .map(|v| v.iter().filter(|v| v != &&0).collect::<Vec<&usize>>())
            .collect::<Vec<Vec<&usize>>>();
        let col = col
            .iter()
            .map(|val| {
                val.iter()
                    .enumerate()
                    .map(|(i, v)| **v * 10_usize.pow(val.len() as u32 - i as u32 - 1))
                    .sum::<usize>()
            })
            .collect::<Vec<usize>>();
        let sym = &data.symbols[i];
        let val = sym.apply(&col);
        total += val;
    }
    Ok(total as u64)
}

pub fn part2(data: &MultGrid) -> Result<u64, Report> {
    let mut total = 0;
    for i in 0..data.width {
        let mut matrix = data.get_num_col(i);
        let rows = matrix.len();
        let cols = matrix[0].len();

        let new = (0..cols)
            .rev()
            .map(|col| (0..rows).map(|row| matrix[row][col]).collect::<Vec<_>>())
            .map(|v| v.into_iter().filter(|v| v != &0).collect::<Vec<usize>>())
            .collect::<Vec<Vec<_>>>();

        let new = new
            .iter()
            .map(|val| {
                val.iter()
                    .enumerate()
                    .map(|(i, v)| *v * 10_usize.pow(val.len() as u32 - i as u32 - 1))
                    .sum::<usize>()
            })
            .collect::<Vec<usize>>();
        let sym = &data.symbols[i];
        let val = sym.apply(&new);
        total += val;
    }
    Ok(total as u64)
}
pub fn data(filepath: &str) -> Result<MultGrid, Report> {
    let file = File::open(filepath)?;
    let bufreader = BufReader::new(file);
    let rangelist = MultGrid::try_from(bufreader.lines())?;
    Ok(rangelist)
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::Criterion;
    use criterion_macro::criterion;

    #[criterion]
    fn bench_part1(b: &mut Criterion) {
        let data = data("inputs/day06.txt").unwrap();
        b.bench_function("day06-part1", |b| b.iter(|| part1(&data).unwrap()));
    }
    #[criterion]
    fn bench_part2(b: &mut Criterion) {
        let data = data("inputs/day06.txt").unwrap();
        b.bench_function("day06-part2", |b| b.iter(|| part2(&data).unwrap()));
    }
}
