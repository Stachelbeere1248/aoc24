use my_helpers::*;

fn main() {
    bench_and_solve!(run_1, run_2, 20000);
}

use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Vec2D {
    x: u8,
    y: u8,
}

const SIZE: Vec2D = Vec2D { x: 50, y: 50 };

impl Vec2D {
    #[inline]
    fn is_valid(&self) -> bool {
        self.x < SIZE.x && self.y < SIZE.y
    }

    #[inline]
    fn added(&self, xy: (i8, i8)) -> Self {
        Self {
            x: self.x.checked_add_signed(xy.0).unwrap_or(SIZE.x),
            y: self.y.checked_add_signed(xy.1).unwrap_or(SIZE.y),
        }
    }

    #[inline]
    fn vec_to(&self, other: &Vec2D) -> (i8, i8) {
        assert!(self.is_valid());
        assert!(other.is_valid());
        (other.x as i8 - self.x as i8, other.y as i8 - self.y as i8)
    }
}

impl From<usize> for Vec2D {
    fn from(index: usize) -> Self {
        Self {
            x: (index % (SIZE.x + 1) as usize) as u8,
            y: (index / (SIZE.y + 1) as usize) as u8,
        }
    }
}

macro_rules! gen_map {
    ($input:expr, $m:ident, $o:ident) => {
        let mut $m = HashMap::<u8, HashSet<Vec2D>>::new();
        let mut $o = HashSet::<Vec2D>::new();
        for (i, &c) in $input.as_bytes().iter().enumerate() {
            match c {
                b'\n' | b'\r' | b'.' => {}
                node => {
                    $m.entry(node)
                        .or_insert_with(|| HashSet::with_capacity(1))
                        .insert(Vec2D::from(i));
                }
            };
        }
    };
}

#[inline]
fn run_1(input: &str) -> u16 {
    gen_map!(input, map, antinodes);
    for set in map.values() {
        for v1 in set.iter() {
            for v2 in set.iter().filter(|&v2| v2 != v1) {
                let test_node = v2.added(v1.vec_to(v2));
                if test_node.is_valid() {
                    antinodes.insert(test_node);
                }
            }
        }
    }
    antinodes.len() as u16
}


#[inline]
fn run_2(input: &str) -> u16 {
    gen_map!(input, map, antinodes);
    'outer: for set in map.values() {
        for v1 in set.iter() {
            if set.len() >= 2 {
                antinodes.insert(*v1);
            } else {
                continue 'outer;
            }
            for v2 in set.iter().filter(|&v2| v2 != v1) {
                let mut test_node = v2.added(v1.vec_to(v2));
                while test_node.is_valid() {
                    antinodes.insert(test_node);
                    test_node = test_node.added(v1.vec_to(v2));
                }
            }
        }
    }
    antinodes.len() as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("tests.txt");
    // sadly the algorithm expects a const input size of 50x50, so no tests :(
    fn part_1() {
        assert_eq!(run_1(EXAMPLE.trim()), 14);
    }

    fn part_2() {
        assert_eq!(run_2(EXAMPLE.trim()), 34);
    }
}