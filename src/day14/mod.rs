use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use crate::day14::Tile::{Air, Rock, Sand};

const INPUT: &str = include_str!("input.txt");

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
enum Tile {
  Rock,
  Air,
  Sand,
}

impl Display for Tile {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Rock => "🪨",
      Air => "  ",
      Sand => "🥪",
    })
  }
}

#[derive(Debug, Default)]
struct Cave {
  tiles: Vec<Vec<Tile>>,
  height: usize,
  correction_factor: usize,
}

impl Display for Cave {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for y in self.tiles.iter() {
      for x in y {
        write!(f, "{x}")?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

impl Cave {
  fn drop_sand(&mut self) -> bool {
    let (mut x, mut y) = (500 - self.correction_factor, 0);

    loop {
      if let Some(m) = self.try_move((x, y)) {
        x = m.0;
        y = m.1;
        if y == self.height - 1 {
          return false;
        }
      } else {
        self.tiles[y][x] = Sand;
        if x == (500 - self.correction_factor) && y == 0 {
          return false;
        }
        return true;
      }
    }
  }

  fn try_move(&self, coord: Coord) -> Option<Coord> {
    for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
      let dx = (coord.0 as isize + dx) as usize;
      let dy = (coord.1 as isize + dy) as usize;
      let m = self.tiles.get(dy).and_then(|col| col.get(dx));
      if m.is_some() && !matches!(self.tiles[dy][dx], Rock | Sand) {
        return Some((dx, dy));
      }
    }
    None
  }

  fn add_floor(&mut self) {
    let floor = self.height - 1;

    for tile in self.tiles[floor].iter_mut() {
      *tile = Rock;
    }
  }
}

fn parse_usize<S: AsRef<str>>(s: S) -> usize {
  s.as_ref().trim().parse().ok().unwrap()
}

fn parse_coords(input: (&str, &str)) -> Coord {
  (parse_usize(input.0), parse_usize(input.1))
}

fn parse_cave() -> Cave {
  let mut points = HashSet::new();
  for line in INPUT.lines() {
    let mut coords = Vec::new();
    for coord in line.split(" -> ") {
      let (x, y) = coord
        .split_once(',')
        .map(parse_coords)
        .unwrap();
      coords.push((x, y));
    }
    for pair in coords.iter().zip(coords.get(1..).unwrap()) {
      let (x1, y1) = pair.0;
      let (x2, y2) = pair.1;
      for x in *x1.min(x2)..=*x2.max(x1) {
        for y in *y1.min(y2)..=*y2.max(y1) {
          points.insert((x, y));
        }
      }
    }
  }

  let min_x = points.iter().map(|(x, _)| x).min().unwrap() - 160;
  let max_x = points.iter().map(|(x, _)| x).max().unwrap() + 160;
  let height = points.iter().map(|(_, y)| y).max().unwrap() + 3;
  let width = max_x - min_x + 2;
  let mut cave = Cave {
    height,
    correction_factor: min_x,
    tiles: vec![vec![Air; width]; height],
  };

  for (x, y) in points.iter() {
    cave.tiles[*y][*x - min_x] = Rock;
  }

  cave
}

pub fn part_one() -> usize {
  let mut cave = parse_cave();
  let mut total = 0;
  while cave.drop_sand() { total += 1 }
  total
}

pub fn part_two() -> usize {
  let mut cave = parse_cave();
  cave.add_floor();
  let mut total = 0;
  while cave.drop_sand() { total += 1 }
  total + 1
}

/*
┏━━━━ DAY 14 ━━━━┓
┃638             ┃
┃31722           ┃
┗━━━━━━━━━━━━━━━━┛
 */