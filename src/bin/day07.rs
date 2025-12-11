use aoc_2025::day07;
use color_eyre::Report;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let data = day07::data("inputs/day07.txt")?;
    let code1 = day07::part1(&data)?;
    let code2 = day07::part2(&data)?;
    println!("code1: {code1}");
    println!("code2: {code2}");
    Ok(())
}
