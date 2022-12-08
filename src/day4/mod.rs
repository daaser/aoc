use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn parse_line_to_set(line: &str) -> (HashSet<usize>, HashSet<usize>) {
  // let (r1_start, r1_end, r2_start, r2_end): (usize, usize, usize, usize);
  // text_io::scan!(line.bytes() => "{}-{},{}-{}", r1_start, r1_end, r2_start, r2_end);

  let range: Vec<usize> = line.split(&[',', '-']).filter_map(|n| n.parse().ok()).collect();
  let [r1_start, r1_end, r2_start, r2_end] = range[..] else { unreachable!() };
  ((r1_start..=r1_end).collect(), (r2_start..=r2_end).collect())
}

pub fn part1() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let (r1, r2) = parse_line_to_set(line);
    if r1.is_subset(&r2) || r2.is_subset(&r1) {
      total += 1;
    }
  }
  total
}

pub fn part2() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let (r1, r2) = parse_line_to_set(line);
    if !r1.is_disjoint(&r2) {
      total += 1;
    }
  }
  total
}

/*
┏━━━━ DAY 4  ━━━━┓
┃413             ┃
┃806             ┃
┗━━━━━━━━━━━━━━━━┛
 */
