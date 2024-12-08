use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};

macro_rules! invalid_index {
    ($i:expr, $size:expr) => {
        $i.0 >= $size.0 || $i.1 >= $size.1
    };
}


fn main() {
    let mut input = String::new();
    File::open("../input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut output = 0_u16;
    let mut_map = unsafe { input.as_bytes_mut() };
    for i in 0..mut_map.len() {
        let orig = mut_map[i];
        if orig == 0x2E || mut_map[i] == 0x23 {
            mut_map[i] = 0x23;
            if is_loop(mut_map, START_DIR, START, SIZE) {
                output += 1;
            }
            mut_map[i] = orig;
        }
    }

    File::create("output.txt")
        .unwrap()
        .write_all(output.to_string().as_bytes())
        .unwrap();
}

#[inline]
pub fn is_loop(input: &[u8], start_dir: usize, start_pos: (u8, u8), size: (u8, u8)) -> bool {
    let mut dir = start_dir;
    let mut pos = start_pos;
    let mut reached: HashSet<((u8, u8), usize)> = HashSet::from([(start_pos, dir)]);
    loop {
        pos = (pos.0.checked_add_signed(DIRS[dir].0).unwrap_or(255), pos.1.checked_add_signed(DIRS[dir].1).unwrap_or(255));
        if invalid_index!(pos, size) { break; }
        unsafe {
            if *(*input).as_ptr().add((pos.0 as usize * (size.1 + 1) as usize) + pos.1 as usize) != 0x23_u8 {
                if !reached.insert(((pos.0, pos.1), dir)) { return true; }
            } else {
                pos = (pos.0.checked_add_signed(-DIRS[dir].0).unwrap_or(255), pos.1.checked_add_signed(-DIRS[dir].1).unwrap_or(255));
                dir = (dir + 1) % 4;
            }
        }
    }
    false
}

const SIZE: (u8, u8) = (130, 130);
const START: (u8, u8) = (86, 45);
const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const START_DIR: usize = 0;

#[test]
fn test_start() {
    let mut input = String::from("....#.....
.........#
..........
..#.......
.......#..
..........
.#......v.
........#.
#.........
......#...");
    let t = is_loop(input.as_bytes(), 2, (6, 8), (10, 10));
    assert_eq!(t, false);
}
