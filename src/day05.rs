use color_eyre::{
    Report,
    eyre::{ContextCompat, eyre},
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    iter::Enumerate,
    rc::Rc,
};
#[derive(Debug)]
pub struct RangeList {
    ranges: Vec<(usize, usize)>,
    data: Vec<usize>,
}

impl<R: BufRead> TryFrom<Lines<R>> for RangeList {
    type Error = Report;
    fn try_from(value: Lines<R>) -> Result<Self, Self::Error> {
        let mut step = 0;
        let mut ranges = Vec::new();
        let mut data = Vec::new();
        for line in value {
            let line = line?;
            match step {
                0 => {
                    let parts = line.split('-').collect::<Vec<&str>>();
                    if parts.len() != 2 {
                        step = 1;
                        continue;
                    }
                    let start: usize = parts[0].parse()?;
                    let end: usize = parts[1].parse()?;
                    ranges.push((start, end));
                }
                1 => {
                    let number: usize = line.parse()?;
                    data.push(number);
                }
                _ => {}
            }
        }
        Ok(Self { ranges, data })
    }
}

impl RangeList {
    pub fn in_range2<'a>(ranges: &'a [(usize, usize)], v: &'a usize) -> Option<&'a (usize, usize)> {
        ranges.iter().find(|(start, end)| v >= start && v <= end)
    }
    pub fn in_range_except2<'a>(
        ranges: &'a [(usize, usize)],
        v: &'a usize,
        range: (&'a usize, &'a usize),
    ) -> Option<&'a (usize, usize)> {
        ranges.iter().find(|(start, end)| {
            // println!(
            //     "2: {start:02}-{end:02} {range:?} {} {}",
            //     !(start != range.0 && end != range.1),
            //     v >= start && v <= end
            // );

            (v >= start && v <= end) && !(start == range.0 && end == range.1)
        })
    }
    pub fn in_range(&self, v: &usize) -> Option<&(usize, usize)> {
        self.ranges
            .iter()
            .find(|(start, end)| v >= start && v <= end)
    }
    pub fn in_range_except(&self, v: &usize, range: (&usize, &usize)) -> Option<&(usize, usize)> {
        self.ranges.iter().find(|(start, end)| {
            // println!(
            //     "2: {start:02}-{end:02} {range:?} {} {}",
            //     !(start != range.0 && end != range.1),
            //     v >= start && v <= end
            // );

            (v >= start && v <= end) && !(start == range.0 && end == range.1)
        })
    }
    pub fn dedup_ranges(&self) -> Vec<(usize, usize)> {
        let mut ranges = self.ranges.clone();
        let mut index = 0;
        while index < ranges.len() {
            let Some((start, end)) = ranges.get(index) else {
                break;
            };
            let (start, end) = (*start, *end);

            // println!("START -- {start}-{end}");
            let has_start = Self::in_range_except2(&ranges, &start, (&start, &end)).copied();
            let has_end = Self::in_range_except2(&ranges, &end, (&start, &end)).copied();
            // println!("{has_start:?}-{has_end:?}");
            let mut reset = false;
            match (has_start, has_end) {
                (Some(new_start), None) if new_start.0 != start => {
                    ranges.remove(index);
                    ranges.insert(0, (new_start.0, end));
                    reset = true;
                }
                (None, Some(new_end)) if new_end.1 != end => {
                    ranges.remove(index);
                    ranges.insert(0, (start, new_end.1));
                    reset = true;
                }
                (Some(new_start), Some(new_end)) if new_start != new_end => {
                    let new = (new_start.0, new_end.1);
                    if new != (start, end) {
                        ranges.remove(index);
                        ranges.insert(0, new);
                        reset = true;
                    }
                }
                (Some(new_start), Some(new_end)) if new_start == new_end => {
                    ranges.remove(index);
                    reset = true;
                }
                _ => {
                    // panic!("unhandled")
                }
            }
            if reset {
                index = 0;
            } else {
                index += 1;
            }
            // println!("END -- {start}-{end} -- {ranges:?}");
        }

        // println!("{ranges:?}");

        ranges.sort();
        ranges.dedup();
        ranges
    }
}

pub fn part1(data: &RangeList) -> Result<u64, Report> {
    let mut total = 0;
    for i in &data.data {
        total += if data.in_range(i).is_some() { 1 } else { 0 };
    }
    Ok(total)
}

pub fn part2(data: &RangeList) -> Result<u64, Report> {
    let ranges = data.dedup_ranges();
    let mut total = 0;
    for (start, end) in &ranges {
        let v = end - start + 1;
        // println!("{start}-{end} => {v}");
        total += v;
    }
    Ok(total as u64)
}
pub fn data(filepath: &str) -> Result<RangeList, Report> {
    let file = File::open(filepath)?;
    let bufreader = BufReader::new(file);
    let rangelist = RangeList::try_from(bufreader.lines())?;
    Ok(rangelist)
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::Criterion;
    use criterion_macro::criterion;

    #[criterion]
    fn bench_part1(b: &mut Criterion) {
        let data = data("inputs/day05.txt").unwrap();
        b.bench_function("day05-part1", |b| b.iter(|| part1(&data).unwrap()));
    }
    #[criterion]
    fn bench_part2(b: &mut Criterion) {
        let data = data("inputs/day05.txt").unwrap();
        b.bench_function("day05-part2", |b| b.iter(|| part2(&data).unwrap()));
    }
}
