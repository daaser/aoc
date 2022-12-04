use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let range: Vec<usize> = line.split(&[',', '-']).map(|n| n.parse::<usize>().unwrap()).collect();
    let [r1_start, r1_end, r2_start, r2_end] = range[..] else { unreachable!() };
    let r1: HashSet<usize> = (r1_start..=r1_end).collect();
    let r2: HashSet<usize> = (r2_start..=r2_end).collect();
    if r1.is_subset(&r2) || r2.is_subset(&r1) {
      total += 1;
    }
  }
  total
}

pub fn part2() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let range: Vec<usize> = line.split(&[',', '-']).map(|n| n.parse::<usize>().unwrap()).collect();
    let [r1_start, r1_end, r2_start, r2_end] = range[..] else { unreachable!() };
    let r1: HashSet<usize> = (r1_start..=r1_end).collect();
    let r2: HashSet<usize> = (r2_start..=r2_end).collect();
    if !r1.is_disjoint(&r2) {
      total += 1;
    }
  }
  total
}

/*
=== DAY 4  ====
413
806
===============
 */
