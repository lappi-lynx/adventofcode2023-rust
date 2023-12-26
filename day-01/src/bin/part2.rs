use miette::{Result,Error,Context};

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();
    let input = include_str!("./input1.txt");
    let result = process(input).context("process part 2")?;
    dbg!(result);
    Ok(())
}

pub fn process(input: &str) -> Result<String, Error> {
    let result = input
        .lines()
        .map(process_line).sum::<u32>();
    Ok(result.to_string())
}

const SPELLED_NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn process_line(line: &str) -> u32 {
    let mut result = 0;
    let mut parsed_digits = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    for (index, &char) in chars.iter().enumerate() {
        if char.is_digit(10) {
            parsed_digits.push(char.to_string())
        }

        for (i, word) in SPELLED_NUMBERS.iter().enumerate() {
            if line[index..].starts_with(word) {
                parsed_digits.push((i+1).to_string());
            }
        }
    }

    if let (Some(&ref first), Some(&ref last)) = (parsed_digits.first(), parsed_digits.last()) {
        let concatenated = format!("{}{}", first, last);
        result += concatenated.parse::<u32>().unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    fn line_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
