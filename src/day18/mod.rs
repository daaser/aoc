use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::thread::sleep;
use std::time::Duration;

const INPUT: &str = include_str!("input.txt");
const NEIGHBORS: [(isize, isize, isize); 6] =
  [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cube {
  x: isize,
  y: isize,
  z: isize,
}

impl Ord for Cube {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl PartialOrd for Cube {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.x < other.x && self.y < other.y && self.z < other.z {
      Some(Ordering::Less)
    } else if self.x > other.x && self.y > other.y && self.z > other.z {
      Some(Ordering::Greater)
    } else if self.x == other.x && self.y == other.y && self.z == other.z {
      Some(Ordering::Equal)
    } else {
      None
    }
  }

  fn lt(&self, other: &Self) -> bool {
    self.x < other.x && self.y < other.y && self.z < other.z
  }

  fn le(&self, other: &Self) -> bool {
    self.x <= other.x && self.y <= other.y && self.z <= other.z
  }

  fn gt(&self, other: &Self) -> bool {
    self.x > other.x && self.y > other.y && self.z > other.z
  }

  fn ge(&self, other: &Self) -> bool {
    self.x >= other.x && self.y >= other.y && self.z >= other.z
  }
}

impl Cube {
  pub fn neighbors(&self) -> [Self; 6] {
    [
      Cube { x: self.x - 1, y: self.y, z: self.z },
      Cube { x: self.x + 1, y: self.y, z: self.z },
      Cube { x: self.x, y: self.y + 1, z: self.z },
      Cube { x: self.x, y: self.y - 1, z: self.z },
      Cube { x: self.x, y: self.y, z: self.z + 1 },
      Cube { x: self.x, y: self.y, z: self.z - 1 },
    ]
  }
}

fn parse_cubes() -> HashSet<Cube> {
  let mut cubes = HashSet::new();
  for line in INPUT.lines() {
    let (x, y, z): (isize, isize, isize);
    text_io::scan!(line.bytes() => "{},{},{}", x, y, z);
    cubes.insert(Cube { x, y, z });
  }
  cubes
}

pub fn part_one() -> isize {
  let mut exposed_sides = 0;
  let cubes = parse_cubes();
  for cube in &cubes {
    for neighbor in cube.neighbors() {
      if !cubes.contains(&neighbor) {
        exposed_sides += 1
      }
    }
  }
  exposed_sides
}

pub fn part_two() -> isize {
  let cubes = parse_cubes();
  let mut droplet = VecDeque::new();
  let mut visited = HashSet::new();

  let min_cube = Cube { x: -1, y: -1, z: -1 };
  let max_cube = cubes.iter().fold(min_cube, |acc, c| Cube {
    x: acc.x.max(c.x + 1),
    y: acc.y.max(c.y + 1),
    z: acc.z.max(c.z + 1),
  });

  droplet.push_back(min_cube);
  let mut exposed_sides = 0isize;
  while let Some(cube) = droplet.pop_front() {
    if !visited.contains(&cube) {
      visited.insert(cube);
      for neighbor in cube.neighbors().into_iter().filter(|n| n >= &min_cube && n <= &max_cube) {
        // sleep(Duration::from_millis(500));
        // println!("{:?}", neighbor);
        if cubes.contains(&neighbor) {
          exposed_sides += 1;
        } else {
          droplet.push_back(neighbor);
        }
      }
    }
  }
  exposed_sides
}

/*
┏━━━━ DAY 18 ━━━━┓
┃4460            ┃
┃2498            ┃
┗━━━━━━━━━━━━━━━━┛
 */
