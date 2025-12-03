use aoc_2025::day02;
use color_eyre::Report;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let data = day02::data("inputs/day02.txt")?;
    let code1 = day02::part1(&data)?;
    let code2 = day02::part2(&data)?;
    println!("code1: {code1}");
    println!("code2: {code2}");
    Ok(())
}
