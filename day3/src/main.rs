use my_helpers::*;
use regex::Regex;

fn main() {
    bench_and_solve!(run_1, run_2, 200);
}

#[inline]
fn run_1(input: &str) -> u32 {
    let matcher: Regex = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
    matcher
        .captures_iter(input)
        .map(|c| (c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap()))
        .map(|(a, b)| a * b)
        .sum::<u32>()
}

#[inline]
fn run_2(input: &str) -> usize {
    let text = format!("do(){input}don't()");
    let filter = Regex::new(r"(?s)do\(\).*?don't\(\)").unwrap();
    let matcher = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    filter
        .find_iter(&text)
        .flat_map(|s| {
            matcher
                .captures_iter(s.as_str())
                .map(|c| (c[1].parse::<u16>().unwrap(), c[2].parse::<u16>().unwrap()))
                .map(|(a, b)| {
                    a as u32 * b as u32
                })
        })
        .sum::<u32>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("tests.txt");

    #[test]
    fn part_1() {
        assert_eq!(run_1(EXAMPLE), 161);
    }

    #[test]
    fn part_2() {
        assert_eq!(run_2(EXAMPLE), 48);
    }
}