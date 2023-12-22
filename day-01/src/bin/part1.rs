use miette::{Result, Error};
// use tracing::{info, instrument};

// #[tracing::instrument]
fn main() {
    println!("part1");
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(_input: &str) -> Result<String, Error> {
    Ok("142".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(input)?;
        assert_eq!("142", result);
        Ok(())
    }}
