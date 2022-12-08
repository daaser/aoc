use std::char::ParseCharError;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Grid {
  array: Vec<Vec<usize>>,
  height: usize,
  width: usize,
}

impl FromStr for Grid {
  type Err = ParseCharError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut arr: Vec<Vec<usize>> = vec![];
    for line in s.lines() {
      let line = line.split("").filter_map(|c| c.parse().ok());
      arr.push(line.collect());
    }
    let height = arr.len();
    let width = arr[0].len();
    Ok(Self { array: arr, height, width })
  }
}

impl Grid {
  fn is_visible(&self, x: usize, y: usize) -> bool {
    if self.is_edge(x, y) {
      return true;
    }
    let target = self.array[y][x];
    let (mut left, mut right, mut top, mut bottom) = (true, true, true, true);
    for i in 0..x {
      if self.array[y][i] >= target {
        left = false
      }
    }
    for i in x + 1..self.width {
      if self.array[y][i] >= target {
        right = false
      }
    }
    for i in 0..y {
      if self.array[i][x] >= target {
        top = false
      }
    }
    for i in y + 1..self.height {
      if self.array[i][x] >= target {
        bottom = false;
      }
    }
    left || right || top || bottom
  }

  fn scenic_score(&self, x: usize, y: usize) -> usize {
    if self.is_edge(x, y) {
      return 0;
    }
    let target = self.array[y][x];
    let (mut left, mut right, mut top, mut bottom) = (0, 0, 0, 0);
    for i in (0..x).rev() {
      left += 1;
      if self.array[y][i] >= target {
        break
      }
    }
    for i in x + 1..self.width {
      right += 1;
      if self.array[y][i] >= target {
        break
      }
    }
    for i in (0..y).rev() {
      top += 1;
      if self.array[i][x] >= target {
        break
      }
    }
    for i in y + 1..self.height {
      bottom += 1;
      if self.array[i][x] >= target {
        break
      }
    }
    left * right * top * bottom
  }

  fn is_edge(&self, x: usize, y: usize) -> bool {
    if x == self.width - 1 || x == 0 || y == self.height - 1 || y == 0 {
      true
    } else {
      false
    }
  }
}

pub fn part1() -> usize {
  let mut total = 0;
  let g = Grid::from_str(INPUT).unwrap();
  for y in 0..g.height {
    for x in 0..g.width {
      if g.is_visible(x, y) {
        total += 1
      }
    }
  }
  total
}

pub fn part2() -> usize {
  let mut score = 0;
  let g = Grid::from_str(INPUT).unwrap();
  for y in 0..g.height {
    for x in 0..g.width {
      let ss = g.scenic_score(x, y);
      score = score.max(g.scenic_score(x, y));
    }
  }
  score
}

/*
┏━━━━ DAY 8  ━━━━┓
┃1851            ┃
┃574080          ┃
┗━━━━━━━━━━━━━━━━┛
 */