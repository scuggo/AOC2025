use aoc_2025::day06;
use color_eyre::Report;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let data = day06::data("inputs/day06example.txt")?;
    let code1 = day06::part1(&data)?;
    let code2 = day06::part2(&data)?;
    println!("code1: {code1}");
    println!("code2: {code2}");
    Ok(())
}
