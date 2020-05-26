use std::error::Error;
use std::io::{self, Read};
type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}
fn part1(input: &str) -> Result<()> {
    let modules_mass: i32 = input
        .lines()
        .map(|mass| mass.parse::<i32>().unwrap() / 3 - 2)
        .sum();
    println!("{}", modules_mass);
    Ok(())
}
fn part2(input: &str) -> Result<()> {
    let mut modules_mass: Vec<i32> = input
        .lines()
        .map(|mass| mass.parse::<i32>().unwrap())
        .collect();
    let mut sum: i32 = 0;

    for v in modules_mass.iter_mut() {
        loop {
            *v = *v / 3 - 2;
            sum += if *v > 0 { *v } else { break }
        }
    }

    println!("{}", sum);
    Ok(())
}
