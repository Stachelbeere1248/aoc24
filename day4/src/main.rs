use my_helpers::*;

fn main() {
    bench_and_solve!(run_1, run_2, 20);
}

macro_rules! invalid_index {
    ($i:expr, $j:expr, $size_i:expr, $size_j:expr) => {
        $i >= $size_i || $i < 0_isize || $j >= $size_j || $j < 0_isize
    };
}
macro_rules! inc_index {
    ($i:expr, $j:expr, $deltas:expr, $sign:expr) => {
        ($i + ($deltas.0 * $sign), $j + ($deltas.1 * $sign))
    };
}

fn run_1(input: &str) -> u16 {
    const MAS: [u8; 3] = [0x4D, 0x41, 0x53];
    const AXIS: [(isize, isize); 8] = [
        (1, 0),
        (0, 1),
        (1, 1),
        (1, -1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
    ];

    let rows = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|&c| c).collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    let row_count = rows.len();
    let row_len = rows[0].len();

    let mut xmas_count = 0_u16;

    for i in 0_usize..row_count {
        for j in 0_usize..row_len {
            if rows[i][j] == 0x58 {
                'dirs: for axis in AXIS {
                    let (mut i1, mut j1) = (i as isize, j as isize);
                    for check in MAS {
                        (i1, j1) = inc_index!(i1, j1, axis, 1);
                        if invalid_index!(i1, j1, row_count as isize, row_len as isize)
                            || rows[i1 as usize][j1 as usize] != check
                        {
                            continue 'dirs;
                        };
                    }
                    xmas_count += 1;
                }
            }
        }
    }
    xmas_count
}

fn run_2(input: &str) -> u16 {
    const M: u8 = 0x4D;
    const A: u8 = 0x41;
    const S: u8 = 0x53;
    const AXIS: [(isize, isize); 2] = [(1, 1), (1, -1)];

    let rows = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|&c| c).collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    let row_count = rows.len();
    let row_len = rows[0].len();

    let mut xmas_count = 0_u16;

    for i in 0_usize..row_count {
        'chars: for j in 0_usize..row_len {
            if rows[i][j] == A {
                for axis in AXIS {
                    let ab = [1_isize, -1_isize]
                        .iter()
                        .map(|&inc| {
                            let (i1, j1) = inc_index!(i as isize, j as isize, axis, inc);
                            if invalid_index!(i1, j1, row_count as isize, row_len as isize) {
                                0
                            } else {
                                rows[i1 as usize][j1 as usize]
                            }
                        })
                        .collect::<Vec<u8>>();
                    if !((ab[0] == M && ab[1] == S) || (ab[1] == M && ab[0] == S)) {
                        continue 'chars;
                    }
                }
                xmas_count += 1;
            }
        }
    }
    xmas_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("tests.txt");
    #[test]
    fn part_1() {
        assert_eq!(run_1(EXAMPLE), 18);
    }
    #[test]
    fn part_2() {
        assert_eq!(run_2(EXAMPLE), 9);
    }
}
