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
    data: Vec<usize>,
    symbols: Vec<Symbol>,
    width: usize,
    num_height: usize,
    sym_height: usize,
}

impl<R: BufRead> TryFrom<Lines<R>> for MultGrid {
    type Error = Report;
    fn try_from(value: Lines<R>) -> Result<Self, Self::Error> {
        let mut data = Vec::new();
        let mut symbols = Vec::new();
        let mut width = 0;

        for line in value {
            let line = line?;
            let parts = line.split(' ').collect::<Vec<&str>>();
            width = parts.len();
            for char in parts {
                println!("char: {char:?}");
                let num = char.parse::<usize>();
                match (char, num) {
                    (_, Ok(num)) => {
                        data.push(num);
                    }
                    ("", _) => data.push(0),
                    ("+", _) => symbols.push(Symbol::Add),
                    ("*", _) => symbols.push(Symbol::Mul),
                    _ => {
                        panic!("FUCK");
                    }
                }
            }
        }
        Ok(Self {
            width,
            num_height: data.len() / width,
            sym_height: symbols.len() / width,
            data,
            symbols,
        })
    }
}

impl MultGrid {
    pub fn get_num_col(&self, index: usize) -> Vec<usize> {
        (0..self.num_height)
            .map(|x| (x * self.width) + index)
            .map(|i| self.data[i])
            .collect()
    }
}

pub fn part1(data: &MultGrid) -> Result<u64, Report> {
    let mut total = 0;
    for i in 0..data.width {
        let col = data.get_num_col(i);
        let sym = &data.symbols[i];
        let val = sym.apply(&col);
        total += val;
    }
    Ok(total as u64)
}

pub fn part2(data: &MultGrid) -> Result<u64, Report> {
    println!("{data:?}");
    // let ranges = data.dedup_ranges();
    let mut total = 0;
    for i in 0..data.width {
        let mut col = data
            .get_num_col(i)
            .into_iter()
            .map(|v| {
                let len = (v.checked_ilog10().unwrap_or(0) + 1) as usize;
                let mut digits = vec![0; len];
                let mut temp = v;
                for i in 1..=len {
                    digits[len - i] = temp % 10;
                    temp /= 10;
                }
                digits
            })
            .collect::<Vec<Vec<usize>>>();
        println!("{col:?}");
        let mut vals = Vec::new();
        while !col.is_empty() {
            let mut val = Vec::new();
            for c in col.iter_mut() {
                let v = c.pop();
                if let Some(v) = v {
                    println!("{v}");
                    val.push(v);
                }
            }

            println!("...");
            col.retain(|v| !v.is_empty());

            let num = val
                .iter()
                .enumerate()
                .map(|(i, v)| v * 10_usize.pow(val.len() as u32 - i as u32 - 1))
                .sum::<usize>();
            vals.push(num);
        }

        let sym = &data.symbols[i];
        let val = sym.apply(&vals);

        println!("{col:?} {sym:?} {val}");
        total += val;
    }
    // for (start, end) in &ranges {
    //     let v = end - start + 1;
    //     // println!("{start}-{end} => {v}");
    //     total += v;
    // }
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
