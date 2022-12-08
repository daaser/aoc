mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
}
