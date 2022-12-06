use std::collections::HashSet;

const INPUT: &'static [u8] = include_bytes!("input.txt");

fn unique_window(size: usize) -> usize {
  let mut count = size;
  'window: for pane in INPUT.windows(size) {
    let set = pane.iter().collect::<HashSet<&u8>>();
    if set.len() == size {
      break 'window;
    }
    count += 1;
  }
  count
}

pub fn part1() -> usize { unique_window(4) }

pub fn part2() -> usize { unique_window(14) }

/*
=== DAY 6  ====
1531
2518
===============
 */