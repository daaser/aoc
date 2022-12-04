use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let range: Vec<&str> = line.split(&[',', '-']).collect();
    match range[..] {
      [r1_start, r1_end, r2_start, r2_end] => {
        let r1_start = r1_start.parse::<usize>().unwrap();
        let r1_end = r1_end.parse::<usize>().unwrap();
        let r2_start = r2_start.parse::<usize>().unwrap();
        let r2_end = r2_end.parse::<usize>().unwrap();

        let r1: HashSet<usize> = (r1_start..=r1_end).collect();
        let r2: HashSet<usize> = (r2_start..=r2_end).collect();
        if r1.is_subset(&r2) || r2.is_subset(&r1) {
          total += 1;
        }
      }
      _ => unreachable!(),
    }
  }
  total
}

pub fn part2() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let range: Vec<&str> = line.split(&[',', '-']).collect();
    match range[..] {
      [r1_start, r1_end, r2_start, r2_end] => {
        let r1_start = r1_start.parse::<usize>().unwrap();
        let r1_end = r1_end.parse::<usize>().unwrap();
        let r2_start = r2_start.parse::<usize>().unwrap();
        let r2_end = r2_end.parse::<usize>().unwrap();

        let r1: HashSet<usize> = (r1_start..=r1_end).collect();
        let r2: HashSet<usize> = (r2_start..=r2_end).collect();
        if !r1.is_disjoint(&r2) {
          total += 1;
        }
      }
      _ => unreachable!(),
    }
  }
  total
}
