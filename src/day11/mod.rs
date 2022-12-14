use std::collections::VecDeque;

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

fn parse_u128<S: AsRef<str>>(s: S) -> Option<u128> {
  s.as_ref().trim().parse().ok()
}

fn parse_usize<S: AsRef<str>>(s: S) -> Option<usize> {
  s.as_ref().trim().parse().ok()
}

fn parse_monkey(s: &str) -> Option<Monkey> {
  let mut lines = s.lines();
  lines.next()?;
  let items = lines
    .next()?
    .split_once(':')
    .map(|(_, items)| items.split(", ").filter_map(parse_u128).collect())?;
  let op = lines.next()?.split_once(':').map(|(_, op)| -> Option<Operation> {
    let mut rsp = op.rsplit(' ');
    let Some(num) = parse_u128(rsp.next()?) else {
      return Some(Operation::Square);
    };
    match rsp.next()? {
      "+" => Some(Operation::Add(num)),
      "*" => Some(Operation::Multiply(num)),
      _ => unreachable!(),
    }
  })??; // 🤔🤔🤔
  let test = parse_u128(lines.next()?.rsplit(' ').next()?)?;
  let t = parse_usize(lines.next()?.rsplit(' ').next()?)?;
  let f = parse_usize(lines.next()?.rsplit(' ').next()?)?;
  Some(Monkey { items, op, test, t, f, items_inspected: 0 })
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
  let mut monkeys = INPUT.split("\n\n").filter_map(parse_monkey).collect::<Vec<_>>();
  for _ in 0..20 {
    round(&mut monkeys, false);
  }
  let mut inspections = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<_>>();
  inspections.sort_by(|a, b| b.cmp(a));
  inspections[0] * inspections[1]
}

pub fn part_two() -> u128 {
  let mut monkeys = INPUT.split("\n\n").filter_map(parse_monkey).collect::<Vec<_>>();
  for _ in 0..10000 {
    round(&mut monkeys, true);
  }
  let mut inspections = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<_>>();
  inspections.sort_by(|a, b| b.cmp(a));
  inspections[0] * inspections[1]
}

/*
┏━━━━ DAY 11 ━━━━┓
┃95472           ┃
┃17926061332     ┃
┗━━━━━━━━━━━━━━━━┛
 */
