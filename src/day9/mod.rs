use std::char::ParseCharError;
use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
const MOVES: &[Coord] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

type Coord = (isize, isize);

#[derive(Debug, Clone)]
struct Rope {
  knots: Vec<Coord>,
  tail_pos: HashSet<Coord>,
}

impl Rope {
  fn new(size: usize) -> Self {
    Self { knots: vec![(0, 0); size], tail_pos: HashSet::new() }
  }

  fn move_head(&mut self, d: Direction) {
    let (dx, dy) = MOVES[d as usize];
    let head = self.knots[0];
    self.knots[0] = (head.0 + dx, head.1 + dy);
  }

  fn move_tail(&mut self) {
    let knots_plus_one = self.knots[1..].to_vec();
    for (i, knot) in knots_plus_one.iter().enumerate() {
      let dx = self.knots[i].0 - knot.0;
      let dy = self.knots[i].1 - knot.1;

      if dx.abs() > 1 || dy.abs() > 1 {
        let dx_tail = dx.signum();
        let dy_tail = dy.signum();
        self.knots[i + 1] = (knot.0 + dx_tail, knot.1 + dy_tail);
      }
    }
  }

  fn track_tail(&mut self) {
    self.tail_pos.insert(*self.knots.last().unwrap());
  }

  fn tail_unique_pos(&self) -> usize {
    self.tail_pos.len()
  }

  fn run_sim(&mut self) {
    for line in INPUT.lines() {
      let (direction, steps) = parse_line(line);
      for _ in 0..steps {
        self.move_head(direction);
        self.move_tail();
        self.track_tail();
      }
    }
  }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
  U = 0,
  D,
  L,
  R,
}

impl FromStr for Direction {
  type Err = ParseCharError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s.parse::<char>() {
      Ok('U') => Direction::U,
      Ok('D') => Direction::D,
      Ok('L') => Direction::L,
      Ok('R') => Direction::R,
      _ => unreachable!(),
    })
  }
}

fn parse_line(line: &str) -> (Direction, usize) {
  line
    .split_once(' ')
    .map(|(d, s)| (Direction::from_str(d).unwrap(), s.parse().unwrap()))
    .unwrap()
}

pub fn part_one() -> usize {
  let mut rope = Rope::new(2);
  rope.run_sim();
  rope.tail_unique_pos()
}

pub fn part_two() -> usize {
  let mut rope = Rope::new(10);
  rope.run_sim();
  rope.tail_unique_pos()
}

/*
┏━━━━ DAY 9  ━━━━┓
┃6384            ┃
┃2734            ┃
┗━━━━━━━━━━━━━━━━┛
 */
