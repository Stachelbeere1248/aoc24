use my_helpers::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;
use std::sync::{LazyLock, RwLock};

fn main() {
    bench_and_solve!(run_1, run_2, 20);
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Page {
    page: u8,
}

impl From<u8> for Page {
    fn from(value: u8) -> Self {
        Self { page: value }
    }
}
impl Into<u32> for Page {
    fn into(self) -> u32 {
        self.page as u32
    }
}

static ORD: LazyLock<RwLock<HashSet<(Page, Page)>>> = LazyLock::new(|| RwLock::new(HashSet::new()));

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = ORD.read().unwrap();
        if ord.contains(&(*self, *other)) {
            Some(Ordering::Less)
        } else if ord.contains(&(*other, *self)) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = ORD.read().unwrap();
        if ord.contains(&(*self, *other)) {
            Ordering::Less
        } else if ord.contains(&(*other, *self)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

macro_rules! gen_ord_and_tuples {
    ($input:expr, $list:ident) => {
        let (p1, $list) = $input.split_once("\n\n").unwrap();
        ORD.write().unwrap().try_reserve(p1.len()).unwrap();
        for line in p1.split('\n') {
            let (a_s, b_s) = line.split_once('|').unwrap();
            let (a, b) = (a_s.parse::<u8>().unwrap(), b_s.parse::<u8>().unwrap());
            ORD.write().unwrap().insert((Page::from(a), Page::from(b)));
        }
    };
}

fn run_1(input: &str) -> u32 {
    gen_ord_and_tuples!(input, updates);

    updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|d| Page::from(u8::from_str(d).unwrap()))
                .collect::<Vec<_>>()
        })
        .filter(|l| l.is_sorted())
        .map(|v| v[v.len() / 2])
        .map(|p| <Page as Into<u32>>::into(p))
        .sum::<u32>()
}

fn run_2(input: &str) -> u32 {
    gen_ord_and_tuples!(input, updates);
    updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|d| Page::from(u8::from_str(d).unwrap()))
                .collect::<Vec<_>>()
        })
        .filter(|l| !l.is_sorted())
        .map(|mut v| {
            v.sort();
            v[v.len() / 2]
        })
        .map(|p| <Page as Into<u32>>::into(p))
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::{run_1, run_2};

    const EXAMPLE: &str = include_str!("tests.txt");

    #[test]
    fn part_1() {
        assert_eq!(run_1(EXAMPLE), 143);
    }

    #[test]
    fn part_2() {
        assert_eq!(run_2(EXAMPLE), 123);
    }
}
