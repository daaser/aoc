use crate::day17::Part::*;

const INPUT: &str = include_str!("input.txt");
const MAP_HEIGHT: usize = 128;
const MAP_WIDTH: usize = 7;

enum Part {
  Air,
  Rock,
}

const ROCK_1: [[Part; 4]; 4] =
  [[Rock, Rock, Rock, Rock], [Air, Air, Air, Air], [Air, Air, Air, Air], [Air, Air, Air, Air]];
const ROCK_2: [[Part; 4]; 4] =
  [[Air, Rock, Air, Air], [Rock, Rock, Rock, Air], [Air, Rock, Air, Air], [Air, Air, Air, Air]];
const ROCK_3: [[Part; 4]; 4] =
  [[Rock, Rock, Rock, Air], [Air, Air, Rock, Air], [Air, Air, Rock, Air], [Air, Air, Air, Air]];
const ROCK_4: [[Part; 4]; 4] =
  [[Rock, Air, Air, Air], [Rock, Air, Air, Air], [Rock, Air, Air, Air], [Rock, Air, Air, Air]];
const ROCK_5: [[Part; 4]; 4] =
  [[Rock, Rock, Air, Air], [Rock, Rock, Air, Air], [Air, Air, Air, Air], [Air, Air, Air, Air]];
const ROCKS: [[[Part; 4]; 4]; 5] = [ROCK_1, ROCK_2, ROCK_3, ROCK_4, ROCK_5];

fn parse() {
  let _input = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
  for c in _input.chars() {
    println!("{c}");
  }
}

pub fn part_one() -> usize {
  // parse();
  0
}

pub fn part_two() -> usize {
  0
}
