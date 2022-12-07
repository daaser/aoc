use std::collections::VecDeque;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Instruction {
  num: usize,
  from: usize,
  to: usize,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (num, from, to): (usize, usize, usize);
    text_io::scan!(s.bytes() => "move {} from {} to {}", num, from, to);
    Ok(Self { num, from, to })
  }
}

fn parse_input() -> (Vec<VecDeque<char>>, Vec<Instruction>) {
  let (begin_state, instructions) = INPUT.split_once("\n\n").unwrap();
  let (stack_data, stacks) = begin_state.rsplit_once('\n').unwrap();

  let num_stacks = stacks.split_whitespace().last().unwrap().parse::<usize>().unwrap();

  let instructions: Vec<Instruction> =
    instructions.lines().map(|line| Instruction::from_str(line).unwrap()).collect();

  let mut stacks = vec![VecDeque::new(); num_stacks];
  for line in stack_data.lines() {
    for (i, data) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
      if data[1].is_alphabetic() {
        stacks[i].push_back(data[1]);
      }
    }
  }

  (stacks, instructions)
}

pub fn part1() -> String {
  let (mut stacks, instructions) = parse_input();
  for inst in instructions {
    for _ in 0..inst.num {
      let tmp = stacks[inst.from - 1].pop_front().unwrap();
      stacks[inst.to - 1].push_front(tmp);
    }
  }
  stacks.into_iter().filter_map(|mut stack| stack.pop_front()).collect()
}

pub fn part2() -> String {
  let (mut stacks, instructions) = parse_input();
  for inst in instructions {
    let range = stacks[inst.from - 1].drain(0..inst.num).collect::<VecDeque<_>>();
    for r in range.into_iter().rev() {
      stacks[inst.to - 1].push_front(r);
    }
  }
  stacks.into_iter().filter_map(|mut stack| stack.pop_front()).collect()
}

/*
=== DAY 5  ====
JCMHLVGMG
LVMRWSSPZ
===============
 */
