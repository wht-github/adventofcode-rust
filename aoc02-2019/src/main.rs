use std::io::{self, Read};
type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut intcodes: Vec<i32> = input
        .split(|c| c == ',' || c == '\n' || c == '\r')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    intcodes[1] = 12;
    intcodes[2] = 2;
    for i in (0..intcodes.len()).step_by(4) {
        match intcodes[i] {
            99 => break,
            1 => {
                let index = intcodes[i + 3] as usize;
                intcodes[index] =
                    intcodes[intcodes[i + 1] as usize] + intcodes[intcodes[i + 2] as usize]
            }
            2 => {
                let index = intcodes[i + 3] as usize;
                intcodes[index] =
                    intcodes[intcodes[i + 1] as usize] * intcodes[intcodes[i + 2] as usize]
            }
            _ => (),
        }
    }
    println!("{}", intcodes[0]);
    Ok(())
}
fn part2(input: &str) -> Result<()> {
    let intcodes: Vec<i32> = input
        .split(|c| c == ',' || c == '\n' || c == '\r')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    for noun in 0..100 {
        for verb in 0..100{
            match cal(intcodes.clone(), noun, verb)? {
                -1 => (),
                x => println!("{}", x),
            }
        }
    }
    Ok(())
}
fn cal(mut intcodes: Vec<i32>, noun: i32, verb: i32)  ->Result<i32>{
    intcodes[1] = noun;
    intcodes[2] = verb;
    for i in (0..intcodes.len()).step_by(4) {
        match intcodes[i] {
            99 => break,
            1 => {
                let index = intcodes[i + 3] as usize;
                intcodes[index] =
                    intcodes[intcodes[i + 1] as usize] + intcodes[intcodes[i + 2] as usize];
            }
            2 => {
                let index = intcodes[i + 3] as usize;
                intcodes[index] =
                    intcodes[intcodes[i + 1] as usize] * intcodes[intcodes[i + 2] as usize];
            }
            _ => (),
        }
    }
    if intcodes[0] == 19690720 {
        Ok(100*noun+verb)
    } else{
        Ok(-1)
    }
}
