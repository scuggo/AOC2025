use aoc_2025::day01;
use color_eyre::Report;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let data = day01::data("inputs/day01.txt")?;
    let code1 = day01::part1(&data)?;
    let code2 = day01::part2(&data)?;
    println!("code1: {code1}");
    println!("code2: {code2}");
    Ok(())
}
