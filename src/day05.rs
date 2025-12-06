use color_eyre::{
    Report,
    eyre::{ContextCompat, eyre},
};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    iter::Enumerate,
    rc::Rc,
};
#[derive(Debug)]
pub struct RangeList {
    ranges: RangeIter,
    data: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct RangeIter {
    inner: Vec<(usize, usize)>,
}
#[derive(Debug)]
pub struct MutRangeIterInner {
    inner: Rc<RefCell<Vec<(usize, usize)>>>,
    index: usize,
}

impl<'a> Iterator for MutRangeIterInner {
    type Item = (usize, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.get(self.index) {
            return Some((self.index, item));
        }
        None
    }
}

impl MutRangeIterInner {
    fn get(&self, index: usize) -> Option<(usize, usize)> {
        self.inner.borrow().get(index).map(|v| *v)
    }
    fn in_range_except(&self, v: &usize, range: (&usize, &usize)) -> Option<(usize, usize)> {
        self.inner
            .borrow()
            .iter()
            .find(|(start, end)| {
                // println!(
                //     "2: {start:02}-{end:02} {range:?} {} {}",
                //     !(start != range.0 && end != range.1),
                //     v >= start && v <= end
                // );

                (v >= start && v <= end) && !(start == range.0 && end == range.1)
            })
            .map(|v| *v)
    }
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
        Ok(Self {
            ranges: RangeIter { inner: ranges },
            data,
        })
    }
}

impl RangeIter {
    pub fn iter_mut(&mut self) -> MutRangeIterInner {
        MutRangeIterInner {
            inner: Rc::new(RefCell::new(self.inner.clone())),
            index: 0,
        }
    }
}

impl RangeList {
    pub fn in_range(&self, v: &usize) -> Option<usize> {
        self.ranges
            .into_iter()
            .position(|(_, (start, end))| v >= &start && v <= &end)
    }
    pub fn in_range_except(&self, v: &usize, range: (&usize, &usize)) -> Option<usize> {
        self.ranges.into_iter().position(|(_, (start, end))| {
            // println!(
            //     "2: {start:02}-{end:02} {range:?} {} {}",
            //     !(start != range.0 && end != range.1),
            //     v >= start && v <= end
            // );

            (v >= start && v <= end) && !(start == range.0 && end == range.1)
        })
    }
    pub fn dedup_ranges(&self) -> Vec<(usize, usize)> {
        let mut old_ranges = self.ranges.clone();
        println!("{:#?}", old_ranges);
        let mut ranges = Vec::new();
        let mut fuck = old_ranges.iter_mut();
        for (int, (start, end)) in fuck {
            println!("START -- {start}-{end}");
            let has_start = fuck.in_range_except(&start, (&start, &end));
            // let has_end = self
            //     .in_range_except(end, (start, end))
            //     .map(|v| self.ranges[v]);
            // println!("{}-{} in {:?}-{:?}", start, end, has_start, has_end);
            // match (has_start, has_end) {
            //     (None, None) => {
            //         let none = (*start, *end);
            //         println!("none: {none:?}");
            //         ranges.push(none);
            //     }
            //     (Some((_, range)), None) => {
            //         let none = (range, *end);
            //         println!("sn: {none:?}");
            //         ranges.push(none);
            //     }
            //     (None, Some((range, _))) => {
            //         let none = (*start, range);
            //         println!("ns: {none:?}");
            //         ranges.push(none);
            //     }
            //     // (Some((_, range_start)), Some((range_end, _))) => {
            //     //     let none = (range_start.min(*start), range_end.max(*end));
            //     //     println!("ss: {none:?}");
            //     //     ranges.push(none);
            //     // }
            //     _ => {}
            // }
        }
        println!("{ranges:?}");

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
    let mut data = RangeList {
        ranges: vec![(3, 5), (5, 8), (3, 4)],
        data: Vec::new(),
    };
    let ranges = data.dedup_ranges();
    // data.ranges = ranges;
    // data.dedup_ranges();
    let mut total = 0;
    for (start, end) in &data.ranges {
        total += end - start;
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
        b.bench_function("day04-part1", |b| b.iter(|| part1(&data).unwrap()));
    }
    #[criterion]
    fn bench_part2(b: &mut Criterion) {
        let data = data("inputs/day05.txt").unwrap();
        b.bench_function("day04-part2", |b| b.iter(|| part2(&data).unwrap()));
    }
}
