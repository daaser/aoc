use std::char::ParseCharError;
use std::str::FromStr;

use crate::day2::Choice::{Paper, Rock, Scissor};

const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Choice {
  Rock(char),
  Paper(char),
  Scissor(char),
}

impl FromStr for Choice {
  type Err = ParseCharError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let c = s.parse::<char>()?;
    match c {
      'A' | 'X' => Ok(Rock(c)),
      'B' | 'Y' => Ok(Paper(c)),
      'C' | 'Z' => Ok(Scissor(c)),
      _ => unreachable!(),
    }
  }
}

impl Choice {
  fn new<T: AsRef<str>>(line: T) -> (Self, Self) {
    let inputs = line
      .as_ref()
      .split_whitespace()
      .map(|c| Choice::from_str(c).unwrap())
      .collect::<Vec<Choice>>();

    (inputs[0], inputs[1])
  }

  fn cmp1(&self, other: Self) -> usize {
    match self {
      Rock('A') if other == Rock('X') => 1 + 3,
      Rock('A') if other == Paper('Y') => 2 + 6,
      Rock('A') if other == Scissor('Z') => 3,
      Paper('B') if other == Rock('X') => 1,
      Paper('B') if other == Paper('Y') => 2 + 3,
      Paper('B') if other == Scissor('Z') => 3 + 6,
      Scissor('C') if other == Rock('X') => 1 + 6,
      Scissor('C') if other == Paper('Y') => 2,
      Scissor('C') if other == Scissor('Z') => 3 + 3,
      _ => unreachable!(),
    }
  }

  // this code is bad and I feel bad
  fn cmp2(&self, other: Self) -> usize {
    match self {
      Rock('A') if other == Rock('X') => 3,
      Rock('A') if other == Paper('Y') => 1 + 3,
      Rock('A') if other == Scissor('Z') => 2 + 6,
      Paper('B') if other == Rock('X') => 1,
      Paper('B') if other == Paper('Y') => 2 + 3,
      Paper('B') if other == Scissor('Z') => 3 + 6,
      Scissor('C') if other == Rock('X') => 2,
      Scissor('C') if other == Paper('Y') => 3 + 3,
      Scissor('C') if other == Scissor('Z') => 1 + 6,
      _ => unreachable!(),
    }
  }
}

pub fn perfect_strategy() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let (theirs, mine) = Choice::new(line);
    total += theirs.cmp1(mine);
  }
  total
}

pub fn perfecter_strategy() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let (theirs, mine) = Choice::new(line);
    total += theirs.cmp2(mine);
  }
  total
}
