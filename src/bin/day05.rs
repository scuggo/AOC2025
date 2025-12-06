use aoc_2025::day05;
use color_eyre::Report;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let data = day05::data("inputs/day05example.txt")?;
    let code1 = day05::part1(&data)?;
    let code2 = day05::part2(&data)?;
    println!("code1: {code1}");
    println!("code2: {code2}");
    Ok(())
}
