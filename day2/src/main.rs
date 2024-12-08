use my_helpers::*;

fn main() {
    bench_and_solve!(run_1, run_2, 200);
}

#[inline]
fn run_1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|a| a.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|list| is_safe(list.iter()))
        .count()
}

#[inline]
fn run_2(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|a| a.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|list| {
            is_safe(list.iter())
                || list
                .iter()
                .find(|&a| is_safe(list.iter().filter(|&e| !std::ptr::eq(a, e))))
                .is_some()
        })
        .count()
}

#[inline]
fn is_safe<'a>(mut iter: impl Iterator<Item=&'a u32>) -> bool {
    let mut second_last = iter.next().unwrap();
    let mut last = iter.next().unwrap();
    for e in iter {
        if !is_within_between(*second_last, *e, *last) {
            return false;
        }
        second_last = last;
        last = e;
    }
    true
}

#[inline]
fn is_within_between(start: u32, end: u32, x: u32) -> bool {
    x.abs_diff(start) <= 3
        && x.abs_diff(end) <= 3
        && if start < end {
        start < x && x < end
    } else if start > end {
        start > x && x > end
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("tests.txt");

    #[test]
    fn part_1() {
        assert_eq!(run_1(EXAMPLE.trim()), 2);
    }

    #[test]
    fn part_2() {
        assert_eq!(run_2(EXAMPLE.trim()), 4);
    }
}





