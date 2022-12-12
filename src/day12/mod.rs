use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");
const MOVES: &[(isize, isize)] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

#[derive(Debug, Clone, Eq, PartialEq)]
struct Grid {
  nodes: Vec<Node>,
  start: usize,
  end: usize,
  width: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
  data: u8,
  pos: usize,
  x: usize,
  y: usize,
}

impl Grid {
  fn add_node(&mut self, data: u8, x: usize, y: usize) {
    let pos = self.nodes.len();
    let node = Node { data, pos, x, y };
    self.nodes.push(node);
  }

  fn bfs(&self, start: usize) -> Option<usize> {
    let mut visited = vec![false; self.nodes.len()];
    let mut heap = VecDeque::new();

    heap.push_back((start, 0));
    visited[self.start] = true;
    while let Some((pos, len)) = heap.pop_front() {
      if pos == self.end {
        return Some(len);
      }
      let curr = &self.nodes[pos];
      for (dx, dy) in MOVES {
        let n = (self.nodes[pos].x as isize + dx)
          + self.width as isize * (self.nodes[pos].y as isize + dy);
        if n < 0 || n >= self.nodes.len() as isize {
          continue;
        }

        let n = n as usize;
        let neighbor = &self.nodes[n];
        if curr.data + 1 >= neighbor.data && !visited[n] {
          visited[n] = true;
          heap.push_back((n, len + 1));
        }
      }
    }
    None
  }
}

fn parse_grid() -> Option<Grid> {
  let mut grid = Grid { nodes: vec![], start: 0, end: 0, width: 0 };
  let mut width = 0usize;
  let mut height = 0usize;
  for line in INPUT.lines() {
    for (idx, c) in line.bytes().enumerate() {
      let data = match c {
        b'S' => {
          grid.start = idx + width;
          b'a'
        }
        b'E' => {
          grid.end = idx + width;
          b'z'
        }
        _ => c,
      };
      grid.add_node(data, idx, height);
    }
    grid.width = line.len();
    width += line.len();
    height += 1;
  }
  Some(grid)
}

pub fn part_one() -> usize {
  let grid = parse_grid().unwrap();
  grid.bfs(grid.start).unwrap()
}

pub fn part_two() -> usize {
  let grid = parse_grid().unwrap();
  let mut shortest = usize::MAX;
  for g in grid.nodes.iter() {
    if g.data == b'a' {
      if let Some(s) = grid.bfs(g.pos) {
        shortest = shortest.min(s);
      }
    }
  }
  shortest
}

/*
┏━━━━ DAY 12 ━━━━┓
┃437             ┃
┃430             ┃
┗━━━━━━━━━━━━━━━━┛
 */
