use my_helpers::*;

fn main() {
    bench_and_solve!(run_1, run_2, 20);
}

macro_rules! parse_line {
    ($line:expr, $l:ident, $r:ident) => {
        let (left, right) = $line.split_once(':').unwrap();
        let ($l, $r) = (
            left.parse::<u64>().unwrap(),
            right
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        );
    };
}

fn run_1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            parse_line!(line, res, numbers);
            (0..2_u32.pow((numbers.len() - 1) as u32))
                .find(|&bitmask| {
                    res == numbers
                        .iter()
                        .skip(1)
                        .zip((0..numbers.len() - 1).map(|i| (bitmask & (1 << i)) != 0))
                        .fold(numbers[0], |acc, (&n2, op)| match op {
                            false => acc + n2,
                            true => acc * n2,
                        })
                })
                .map(|_| res)
        })
        .sum::<u64>()
}

fn run_2(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            parse_line!(line, res, numbers);
            (0..3_u32.pow((numbers.len() - 1) as u32))
                .find(|&bitmask_base3| {
                    res == numbers
                        .iter()
                        .skip(1)
                        .zip(
                            (0..numbers.len() - 1)
                                .map(|i| ((bitmask_base3 / 3_u32.pow(i as u32)) % 3) as u8),
                        )
                        .fold(numbers[0], |acc, (&n2, op)| match op {
                            0 => acc + n2,
                            1 => acc * n2,
                            2 => (acc * 10_u64.pow(n2.checked_ilog10().unwrap() + 1)) + n2,
                            _ => unsafe { std::hint::unreachable_unchecked() },
                        })
                })
                .map(|_| res)
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::{run_1, run_2};

    const EXAMPLE: &str = include_str!("tests.txt");

    #[test]
    fn part_1() {
        assert_eq!(run_1(EXAMPLE), 3749);
    }

    #[test]
    fn part_2() {
        assert_eq!(run_2(EXAMPLE), 11387);
    }
}
