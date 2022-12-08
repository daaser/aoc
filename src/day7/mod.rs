use std::collections::BTreeMap;

use indextree::{Arena, NodeId};

const INPUT: &str = include_str!("input.txt");
const DISK_SIZE: usize = 70_000_000;
const SIZE_NEEDED: usize = 30_000_000;

#[derive(Debug, Clone)]
struct FSNode {
  parent: Option<NodeId>,
  children: BTreeMap<String, NodeId>,
  files: Vec<usize>,
  size: usize,
  _name: String,
}

impl FSNode {
  fn insert_child<K: AsRef<str>>(&mut self, key: K, value: NodeId) {
    self.children.insert(key.as_ref().to_string(), value);
  }
}

fn parse_filesystem(arena: &mut Arena<FSNode>) -> NodeId {
  let root = arena.new_node(FSNode {
    parent: None,
    children: BTreeMap::new(),
    files: vec![],
    size: 0,
    _name: "/".to_string(),
  });
  let mut current = root;

  for line in INPUT.lines() {
    let args = line.split_whitespace().collect::<Vec<_>>();
    if args[0] == "$" && args[1] == "cd" {
      match args[2] {
        "/" => current = root,
        ".." => current = arena[current].get().parent.unwrap(),
        dir => {
          let Some(child) = arena[current].get().children.get(dir) else {
            let new_node = arena.new_node(FSNode {
              parent: Some(current),
              children: BTreeMap::new(),
              files: vec![],
              size: 0,
              _name: dir.to_string(),
            });
            arena[current].get_mut().insert_child(dir, new_node);
            current = new_node;
            continue;
          };
          current = *child;
        }
      }
    } else if args[0] != "$" && args[0] != "dir" {
      let save = current;
      if let Ok(size) = args[0].parse::<usize>() {
        arena[current].get_mut().files.push(size);
        arena[current].get_mut().size += size;
        while let Some(parent) = arena[current].get().parent {
          arena[parent].get_mut().size += size;
          current = parent;
        }
      };
      current = save;
    }
  }
  root
}

pub fn part1() -> usize {
  let arena = &mut Arena::new();
  parse_filesystem(arena);

  let mut total = 0;
  for node in arena.iter() {
    let size = node.get().size;
    if size <= 100_000 {
      total += size;
    }
  }
  total
}

pub fn part2() -> usize {
  let arena = &mut Arena::new();
  let root = parse_filesystem(arena);

  let mut smallest = DISK_SIZE;
  let unused = DISK_SIZE - arena[root].get().size;
  for node in arena.iter() {
    let size = node.get().size;
    if size + unused >= SIZE_NEEDED {
      smallest = smallest.min(size);
    }
  }
  smallest
}

/*
┏━━━━ DAY 7  ━━━━┓
┃1989474         ┃
┃1111607         ┃
┗━━━━━━━━━━━━━━━━┛
 */
