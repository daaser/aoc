mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::time::Instant;

use paste::paste;

macro_rules! day {
  ($day:tt) => {
    let now = Instant::now();
    println!(
      "┏━━━━ DAY {:<2} ━━━━┓\n┃{:<16}┃\n┃{:<16}┃\n┃{:16}┃",
      $day,
      paste! { [<day $day>]::part1() },
      paste! { [<day $day>]::part2() },
      ""
    );
    println!("┃{:<16?}┃\n┗━━━━━━━━━━━━━━━━┛\n", now.elapsed());
  };
}

fn main() {
  day!(1);
  day!(2);
  day!(3);
  day!(4);
  day!(5);
  day!(6);
  day!(7);
}
