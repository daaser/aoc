#![allow(unused)]
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::time::Instant;

use paste::paste;

macro_rules! day {
  ($day:tt) => {
    let now = Instant::now();
    println!(
      "┏━━━━ DAY {:<2} ━━━━┓\n┃{:<16}┃\n┃{:<16}┃\n┃{:16}┃",
      $day,
      paste! { [<day $day>]::part_one() },
      paste! { [<day $day>]::part_two() },
      ""
    );
    if $day == 25 {
      println!("┃{:<16?}┃\n┗━━━━━━━━━━━━━━━━┛", now.elapsed());
    } else {
      println!("┃{:<16?}┃\n┗━━━━━━━━━━━━━━━━┛\n", now.elapsed());
    }
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
  day!(8);
  day!(9);
  day!(10);
  day!(11);
  day!(12);
  day!(13);
  day!(14);
  day!(15);
  day!(16);
  day!(17);
  day!(18);
}
