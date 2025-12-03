use aoc_2025::day03;
use color_eyre::Report;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let data = day03::data("inputs/day03.txt")?;
    let code1 = day03::part1(&data)?;
    let code2 = day03::part2(&data)?;
    println!("code1: {code1}");
    println!("code2: {code2}");
    Ok(())
}
