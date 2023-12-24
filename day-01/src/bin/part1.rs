use miette::{Result,Error,Context};
// use tracing::{info, instrument};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();
    println!("part1");
    let input = include_str!("./input1.txt");
    let result = process(input).context("process part 1")?;
    dbg!(result);
    Ok(())
}

fn process(input: &str) -> Result<String, Error> {
    let result = input
    .lines()
    .map(|line| {
        let mut chars = line.chars()
            .filter_map(|char| {
                char.to_digit(10)
            });
        let first = chars.next().expect("should be a number");

        match chars.last() {
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}"),
        }
        .parse::<u32>()
        .expect("should be a valid number")
    })
    .sum::<u32>();

    Ok(result.to_string())
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
        let result = process(input)?;
        assert_eq!("142", result);
        Ok(())
    }}
