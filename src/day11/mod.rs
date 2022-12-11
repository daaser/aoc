use std::collections::VecDeque;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Operation {
  Multiply(u128),
  Add(u128),
  Square,
}

impl Operation {
  fn perform(&self, old: u128) -> u128 {
    match self {
      Operation::Multiply(op) => old * op,
      Operation::Add(op) => old + op,
      Operation::Square => old * old,
    }
  }
}

#[derive(Debug)]
struct Monkey {
  items: VecDeque<u128>,
  op: Operation,
  test: u128,
  t: usize,
  f: usize,
  items_inspected: u128,
}

impl FromStr for Monkey {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut lines = s.lines();
    lines.next().unwrap();
    let items = lines
      .next()
      .unwrap()
      .split_once(':')
      .map(|(_, items)| items.split(", ").map(|i| i.trim().parse::<u128>().unwrap()).collect())
      .unwrap();
    let op = lines
      .next()
      .unwrap()
      .split_once(':')
      .map(|(_, op)| {
        let mut rsp = op.rsplit(' ');
        let Ok(num) = rsp.next().unwrap().parse::<u128>() else {
          return Operation::Square;
        };
        match rsp.next().unwrap() {
          "+" => Operation::Add(num),
          "*" => Operation::Multiply(num),
          _ => unreachable!(),
        }
      })
      .unwrap();
    let test = lines.next().unwrap().rsplit(' ').next().unwrap().parse::<u128>().unwrap();
    let t = lines.next().unwrap().rsplit(' ').next().unwrap().parse::<usize>().unwrap();
    let f = lines.next().unwrap().rsplit(' ').next().unwrap().parse::<usize>().unwrap();
    Ok(Self { items, op, test, t, f, items_inspected: 0 })
  }
}

fn round(monkeys: &mut Vec<Monkey>, ridiculous: bool) {
  let lcm: u128 = monkeys.iter().map(|m| m.test).product();
  for i in 0..monkeys.len() {
    while let Some(item) = monkeys[i].items.pop_front() {
      monkeys[i].items_inspected += 1;
      let mut worry = monkeys[i].op.perform(item);
      if !ridiculous {
        worry /= 3;
      }
      worry %= lcm;
      if worry % monkeys[i].test == 0 {
        let t = monkeys[i].t;
        monkeys[t].items.push_back(worry);
      } else {
        let f = monkeys[i].f;
        monkeys[f].items.push_back(worry);
      }
    }
  }
}

pub fn part_one() -> u128 {
  let mut monkeys =
    INPUT.split("\n\n").filter_map(|m| Monkey::from_str(m).ok()).collect::<Vec<_>>();
  for _ in 0..20 {
    round(&mut monkeys, false);
  }
  let mut inspections = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<_>>();
  inspections.sort_by(|a, b| b.cmp(&a));
  inspections[0] * inspections[1]
}

pub fn part_two() -> u128 {
  let mut monkeys =
    INPUT.split("\n\n").filter_map(|m| Monkey::from_str(m).ok()).collect::<Vec<_>>();
  for _ in 0..10000 {
    round(&mut monkeys, true);
  }
  let mut inspections = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<_>>();
  inspections.sort_by(|a, b| b.cmp(&a));
  inspections[0] * inspections[1]
}

/*
┏━━━━ DAY 11 ━━━━┓
┃95472           ┃
┃17926061332     ┃
┗━━━━━━━━━━━━━━━━┛
 */