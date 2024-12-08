use my_helpers::*;

fn main() {
    bench_and_solve!(run_1, run_2, 100);
}

macro_rules! gen_lists {
    ($input:expr, $l:ident, $r:ident) => {
        let (mut $l, mut $r) = $input
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unzip::<u32, u32, Vec<_>, Vec<_>>();

        $l.sort();
        $r.sort();
    };
}
fn run_1(input: &str) -> u32 {
    gen_lists!(input, lefts, rights);

    lefts
        .into_iter()
        .zip(rights.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
}

fn run_2(input: &str) -> u32 {
    gen_lists!(input, lefts, rights);

    lefts
        .into_iter()
        .map(|i| i * rights.iter().filter(|&&j| i == j).count() as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("tests.txt");

    #[test]
    fn part_1() {
        assert_eq!(run_1(EXAMPLE), 11);
    }

    #[test]
    fn part_2() {
        assert_eq!(run_2(EXAMPLE), 31);
    }
}
