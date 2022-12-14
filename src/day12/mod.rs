use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");
const MOVES: &[(isize, isize)] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

#[derive(Debug)]
struct Grid {
  nodes: Vec<Node>,
  start: usize,
  end: usize,
  width: isize,
}

#[derive(Debug)]
struct Node {
  data: u8,
  pos: usize,
  x: isize,
  y: isize,
}

impl Grid {
  fn add_node(&mut self, data: u8, x: usize, y: usize) {
    let pos = self.nodes.len();
    #[rustfmt::skip]
    let data = match data {
      b'S' => { self.start = pos; b'a' },
      b'E' => { self.end = pos; b'z' },
      _ => data,
    };
    let node = Node { data, pos, x: x as isize, y: y as isize };
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
        let n = (curr.x + dx) + self.width * (curr.y + dy);
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

fn parse_grid() -> Grid {
  let mut grid = Grid { nodes: vec![], start: 0, end: 0, width: 0 };
  let mut x = 0;
  for line in INPUT.lines() {
    for (idx, ch) in line.bytes().enumerate() {
      grid.add_node(ch, idx, x);
    }
    grid.width = line.len() as isize;
    x += 1;
  }
  grid
}

pub fn part_one() -> usize {
  let grid = parse_grid();
  grid.bfs(grid.start).unwrap()
}

pub fn part_two() -> usize {
  let grid = parse_grid();
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
