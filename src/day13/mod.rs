use serde::Deserialize;
use std::cmp::Ordering;

use crate::day13::Cons::{Car, Cdr};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum Cons {
  Car(u8),
  Cdr(Vec<Cons>),
}

impl PartialOrd for Cons {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (Car(a), Car(b)) => a.partial_cmp(b),
      (Cdr(a), Cdr(b)) => a.partial_cmp(b),
      (Car(a), Cdr(_)) => Cdr(vec![Car(*a)]).partial_cmp(other),
      (Cdr(_), Car(b)) => self.partial_cmp(&Cdr(vec![Car(*b)])),
    }
  }
}

impl Ord for Cons {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl Cons {
  fn from_str(s: &str) -> Self {
    serde_json::from_str::<Cons>(s).unwrap()
  }
}

pub fn part_one() -> usize {
  let mut total = 0;
  for (idx, line) in INPUT.split("\n\n").enumerate() {
    let (left, right) = line.split_once('\n').unwrap();
    let left = Cons::from_str(left);
    let right = Cons::from_str(right);
    if left < right {
      total += idx + 1;
    }
  }
  total
}

pub fn part_two() -> usize {
  let d1 = Cons::from_str("[[2]]");
  let d2 = Cons::from_str("[[6]]");
  let mut all_packets = vec![d1.clone(), d2.clone()];
  for line in INPUT.split('\n').filter(|l| !l.is_empty()) {
    let con = Cons::from_str(line);
    all_packets.push(con);
  }
  all_packets.sort();
  let mut total = 1usize;
  for (idx, packet) in all_packets.into_iter().enumerate() {
    if packet == d1 || packet == d2 {
      total *= idx + 1
    }
  }
  total
}

/*
┏━━━━ DAY 13 ━━━━┓
┃5720            ┃
┃23504           ┃
┗━━━━━━━━━━━━━━━━┛
 */
