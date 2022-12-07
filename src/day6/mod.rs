const INPUT: &[u8] = include_bytes!("input.txt");

#[inline]
fn has_duplicate(arr: &[u8]) -> bool { arr.iter().enumerate().any(|(i, b)| arr[..i].contains(b)) }

fn unique_window(size: usize) -> usize {
  let mut count = size;
  'window: for pane in INPUT.windows(size) {
    if !has_duplicate(pane) {
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
