use std::char::ParseCharError;
use std::str::FromStr;

use crate::day2::Choice::{Paper, Rock, Scissor};
use crate::day2::Outcome::{Draw, Lose, Win};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Choice {
  Rock,
  Paper,
  Scissor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
  Lose,
  Draw,
  Win,
}

impl FromStr for Choice {
  type Err = ParseCharError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let c = s.parse::<char>()?;
    match c {
      'A' | 'X' => Ok(Rock),
      'B' | 'Y' => Ok(Paper),
      'C' | 'Z' => Ok(Scissor),
      _ => unreachable!(),
    }
  }
}

impl FromStr for Outcome {
  type Err = ParseCharError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let c = s.parse::<char>()?;
    match c {
      'X' => Ok(Lose),
      'Y' => Ok(Draw),
      'Z' => Ok(Win),
      _ => unreachable!(),
    }
  }
}

impl Outcome {
  fn new<T: AsRef<str>>(line: T) -> (Choice, Self) {
    let mut inputs = line.as_ref().split_whitespace();
    let choice = Choice::from_str(inputs.next().unwrap()).unwrap();
    let outcome = Outcome::from_str(inputs.next().unwrap()).unwrap();

    (choice, outcome)
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

  fn cmp(&self, other: Self) -> usize {
    match (self, other) {
      (Rock, Rock) => 1 + 3,
      (Rock, Paper) => 2 + 6,
      (Rock, Scissor) => 3,
      (Paper, Rock) => 1,
      (Paper, Paper) => 2 + 3,
      (Paper, Scissor) => 3 + 6,
      (Scissor, Rock) => 1 + 6,
      (Scissor, Paper) => 2,
      (Scissor, Scissor) => 3 + 3,
    }
  }

  // this code is bad and I feel bad
  fn cmp_outcome(&self, other: Outcome) -> usize {
    match (self, other) {
      (Rock, Lose) => 3,
      (Rock, Draw) => 1 + 3,
      (Rock, Win) => 2 + 6,
      (Paper, Lose) => 1,
      (Paper, Draw) => 2 + 3,
      (Paper, Win) => 3 + 6,
      (Scissor, Lose) => 2,
      (Scissor, Draw) => 3 + 3,
      (Scissor, Win) => 1 + 6,
    }
  }
}

pub fn part1() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let (theirs, mine) = Choice::new(line);
    total += theirs.cmp(mine);
  }
  total
}

pub fn part2() -> usize {
  let mut total = 0usize;
  for line in INPUT.lines() {
    let (theirs, mine) = Outcome::new(line);
    total += theirs.cmp_outcome(mine);
  }
  total
}

/*
┏━━━━ DAY 2  ━━━━┓
┃10310           ┃
┃14859           ┃
┗━━━━━━━━━━━━━━━━┛
 */
