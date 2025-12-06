use color_eyre::{
    Report,
    eyre::{ContextCompat, eyre},
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};
#[derive(Clone)]
pub struct Wall {
    inner: Vec<bool>,
    width: usize,
    height: usize,
}

impl<R: BufRead> TryFrom<Lines<R>> for Wall {
    type Error = Report;
    fn try_from(value: Lines<R>) -> Result<Self, Self::Error> {
        let mut width = None;
        let mut height = 1;
        let mut inner: Vec<bool> = Vec::new();
        for line in value {
            let line = line?;
            let chars: Vec<char> = line.trim().chars().collect();
            let new_width = chars.len();
            if let Some(old_width) = width {
                if old_width != new_width {
                    return Err(eyre!("width mismatch"));
                }
            } else {
                width = Some(new_width);
            }
            for c in chars {
                match c {
                    '@' => inner.push(true),
                    '.' => inner.push(false),
                    _ => return Err(Report::msg(format!("Invalid character in wall: {}", c))),
                }
            }
            height += 1;
        }
        height -= 1;
        Ok(Self {
            inner,
            width: width.unwrap(),
            height,
        })
    }
}

impl Wall {
    fn get_neibours(&self, pos: usize) -> Vec<usize> {
        let mut neibours = Vec::new();
        let x = pos % self.width;
        let y = pos / self.width;
        if x > 0 {
            neibours.push(pos - 1);
        }
        if x < self.width - 1 {
            neibours.push(pos + 1);
        }
        if y > 0 {
            neibours.push(pos - self.width);
        }
        if y < self.height - 1 {
            neibours.push(pos + self.width);
        }
        if x > 0 && y > 0 {
            neibours.push(pos - self.width - 1);
        }
        if x < self.width - 1 && y > 0 {
            neibours.push(pos - self.width + 1);
        }
        if x > 0 && y < self.height - 1 {
            neibours.push(pos + self.width - 1);
        }
        if x < self.width - 1 && y < self.height - 1 {
            neibours.push(pos + self.width + 1);
        }

        neibours
    }
    pub fn is_wall(&self, pos: usize) -> bool {
        self.inner[pos]
    }
    pub fn set_wall(&mut self, pos: usize, wall: bool) {
        self.inner[pos] = wall
    }
}

pub fn part1(data: &Wall) -> Result<u64, Report> {
    let mut total = 0;
    for i in 0..data.inner.len() {
        if data.is_wall(i) {
            let test = data
                .get_neibours(i)
                .iter()
                .map(|p| if data.is_wall(*p) { 1 } else { 0 })
                .sum::<usize>();
            if test < 4 {
                total += 1;
            }
        }
    }
    Ok(total)
}

pub fn part2(data: &Wall) -> Result<u64, Report> {
    let mut data = data.clone();
    let mut total = 0;
    let mut changed = true;
    // this is really shit
    while changed {
        changed = false;
        for i in 0..data.inner.len() {
            if data.is_wall(i) {
                let test = data
                    .get_neibours(i)
                    .iter()
                    .map(|p| if data.is_wall(*p) { 1 } else { 0 })
                    .sum::<usize>();
                if test < 4 {
                    data.set_wall(i, false);
                    total += 1;
                    changed = true
                }
            }
        }
    }
    Ok(total)
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
