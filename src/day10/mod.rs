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
  x: isize,
}

impl FromStr for Program {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut ops = vec![];
    for line in s.lines() {
      ops.push(Inst::Noop);
      if !line.starts_with("noop") {
        ops.push(
          line
            .split_once(' ')
            .map(|(_, val)| {
              let val = val.parse::<isize>().unwrap();
              Inst::Addx(val)
            })
            .unwrap(),
        );
      }
    }
    Ok(Self { ops, x: 1 })
  }
}

impl Program {
  fn signal_strength(&mut self) -> isize {
    let mut signal_strength: isize = 0;
    for (cycle, op) in self.ops.iter().enumerate() {
      let cycle = cycle as isize + 1;
      if CYCLES.contains(&cycle) {
        signal_strength += self.x * cycle as isize;
      }
      if let Inst::Addx(val) = op {
        self.x += val;
      }
    }
    signal_strength
  }

  fn render_screen(&mut self) {
    let mut screen = [[false; 40]; 6];
    for (cycle, op) in self.ops.iter().enumerate() {
      let positions = [self.x - 1, self.x, self.x + 1];

      let row = cycle / 40;
      let col = cycle % 40;

      screen[row][col] = positions.contains(&(col as isize));

      if let Inst::Addx(val) = op {
        self.x += val;
      }
    }

    #[rustfmt::skip]
    screen.iter().for_each(|row| {
      row.iter().for_each(|pixel| {
        print!("{}", if *pixel { '\u{2591}' } else { '\u{2593}' })
      }); println!();
    });
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
