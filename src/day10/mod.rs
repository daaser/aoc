use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
const CYCLES: &[isize] = &[20, 60, 100, 140, 180, 220];

#[derive(Debug)]
enum Inst {
  Noop,
  Addx(isize),
}

#[derive(Debug)]
struct Program {
  ops: Vec<Inst>,
  cycle: isize,
  x: isize,
}

impl FromStr for Program {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut ops = vec![];
    for line in s.lines() {
      ops.push(if line.starts_with("noop") {
        Inst::Noop
      } else {
        line
          .split_once(' ')
          .map(|(_, val)| {
            let val = val.parse::<isize>().unwrap();
            Inst::Addx(val)
          })
          .unwrap()
      })
    }
    Ok(Self { ops, x: 1, cycle: 1 })
  }
}

impl Program {
  fn signal_strength(&mut self) -> isize {
    let mut signal_strength: isize = 0;

    for op in self.ops.iter() {
      match op {
        Inst::Addx(val) => {
          for i in 0..2 {
            self.cycle += 1;
            if i == 1 {
              // after cycle
              self.x += val;
            }
            if CYCLES.contains(&self.cycle) {
              signal_strength += self.x * self.cycle;
            }
          }
        }
        Inst::Noop => {
          self.cycle += 1;
          if CYCLES.contains(&self.cycle) {
            signal_strength += self.x * self.cycle;
          }
        }
      }
    }

    signal_strength
  }

  fn update_screen(&self, screen: &mut [[bool; 40]; 6]) {
    let cycle = self.cycle - 1;
    let positions = [self.x - 1, self.x, self.x + 1];

    let row = cycle / 40isize;
    let col = cycle % 40isize;

    let render = positions.contains(&col);
    screen[row as usize][col as usize] = render;
  }

  fn render_screen(&mut self) {
    let mut screen = [[false; 40]; 6];

    for op in self.ops.iter() {
      match op {
        Inst::Addx(val) => {
          for i in 0..2 {
            self.update_screen(&mut screen);
            self.cycle += 1;
            if i == 1 {
              // after cycle
              self.x += val;
            }
            self.update_screen(&mut screen);
          }
        }
        Inst::Noop => {
          self.update_screen(&mut screen);
          self.cycle += 1;
        }
      }
    }

    for y in 0..screen.len() {
      for x in 0..screen[0].len() {
        print!("{}", if screen[y][x] { '\u{2591}' } else { '\u{2593}' });
      }
      println!();
    }
  }
}

pub fn part_one() -> isize {
  let mut prog = Program::from_str(INPUT).unwrap();
  prog.signal_strength()
}

pub fn part_two() -> String {
  let mut prog = Program::from_str(INPUT).unwrap();
  prog.render_screen();
  "".to_string()
}
