use aoc_2025::day04;
use color_eyre::Report;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let data = day04::data("inputs/day04example.txt")?;
    let code1 = day04::part1(&data)?;
    let code2 = day04::part2(&data)?;
    println!("code1: {code1}");
    println!("code2: {code2}");
    Ok(())
}
