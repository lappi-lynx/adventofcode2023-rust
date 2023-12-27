use miette::{Result,Error};

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
    for (index, char) in line.chars().enumerate() {
        if char.is_digit(10) {
            parsed_digits.push(char.to_digit(10).unwrap())
        }

        SPELLED_NUMBERS.iter().enumerate().for_each(|(i, word)| {
            if line[index..].starts_with(word) {
                parsed_digits.push(i as u32 + 1);
            }
        });
    }

    if let (Some(&ref first), Some(&ref last)) = (parsed_digits.first(), parsed_digits.last()) {
        result += first * 10 + last;
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
