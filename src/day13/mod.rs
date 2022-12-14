use std::cmp::Ordering;
use serde::Deserialize;

use crate::day13::Cons::{Car, Cdr};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Ord, Deserialize)]
#[serde(untagged)]
enum Cons {
  Car(u8),
  Cdr(Vec<Cons>),
}

impl PartialOrd for Cons {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    // println!("{:?}\n{:?}\n", self, other);
    match (self, other) {
      (Car(a), Car(b)) => a.partial_cmp(b),
      (Cdr(a), Cdr(b)) => a.partial_cmp(b),
      (Car(a), Cdr(_)) => Cdr(vec![Car(*a)]).partial_cmp(other),
      (Cdr(_), Car(b)) => self.partial_cmp(&Cdr(vec![Car(*b)])),
    }
  }
}

impl Cons {
  fn from_str(s: &str) -> Self {
    serde_json::from_str::<Cons>(s).unwrap()
  }

  // fn parse(s: &str) -> Self {
  //   let mut acc = vec![];
  //   let s = s.trim().replace("10", "a");
  //   let mut i = 0;
  //   let mut bytes = s.bytes();
  //   while let Some(ref b) = bytes.next() {
  //     i += 1;
  //     // println!("{} {:?}", *b as char, acc);
  //     match b {
  //       b'[' => {
  //         acc.push(Cons::parse(&s[i..]));
  //         // if con.is_none() {
  //         //   return None
  //         // }
  //         // return Some(Cdr(acc));
  //       }
  //       b']' => {
  //         if acc.is_empty() {
  //           return None;
  //         }
  //         return Some(Cdr(acc));
  //       },
  //       b',' => {},
  //       b'a' => acc.push(Car(10)),
  //       _ => acc.push(Car(b - b'0')),
  //         // let mut digit = 0u8;
  //         // while let Some(x) = bytes.next() {
  //         //   if !x.is_ascii_digit() {
  //         //     break
  //         //   }
  //         //   println!("before={digit}");
  //         //   digit *= 10;
  //         //   digit += x - b'0';
  //         //   println!("after={digit}");
  //         // }
  //       //   dbg!(acc.push(Car(b - b'0')));
  //       // }
  //     }
  //   }
  //   unreachable!()
  // }
}

pub fn part_one() -> usize {
  let mut total = 0;
  // println!("returned: {:?}", Cons::parse("[[1],[2]]"));
  // println!("returned: {:?}", Cons::parse("[1,[2,[3,[4,[5,6,7]]]],8,9,10]"));
  // println!("returned: {:?}", Cons::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9,10]"));
  println!("returned: {:?}", Cons::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9,10]"));
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
  for line in INPUT.split("\n").filter(|l| !l.is_empty()) {
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