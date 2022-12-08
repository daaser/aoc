use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn calculate_value(b: char) -> usize {
  let b = b as u8;
  let val = match b {
    b'a'..=b'z' => b - b'a' + 1,
    b'A'..=b'Z' => b - b'&',
    _ => 0,
  };
  val as usize
}

pub fn part1() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let line_len = line.len();
    let (first, second) = line.split_at(line_len / 2);

    let f: HashSet<char> = first.chars().collect();
    let s: HashSet<char> = second.chars().collect();

    for int in &f & &s {
      total += calculate_value(int);
    }
  }
  total
}

pub fn part2() -> usize {
  let mut total = 0usize;
  let lines = INPUT.lines().collect::<Vec<_>>();
  let mut chunks = lines.chunks(3);
  while let Some([first, second, third]) = chunks.next() {
    let f: HashSet<char> = first.chars().collect();
    let s: HashSet<char> = second.chars().collect();
    let t: HashSet<char> = third.chars().collect();

    for int in &(&f & &s) & &t {
      total += calculate_value(int)
    }
  }
  total
}

/*
┏━━━━ DAY 3  ━━━━┓
┃7811            ┃
┃2639            ┃
┗━━━━━━━━━━━━━━━━┛
 */
