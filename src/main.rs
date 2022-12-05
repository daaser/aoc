mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use paste::paste;

macro_rules! day {
  ($day:tt) => {
    println!(
      "=== DAY {:<2} ====\n{}\n{}\n===============\n",
      $day,
      paste! { [<day $day>]::part1() },
      paste! { [<day $day>]::part2() },
    )
  };
}

fn main() {
  day!(1);
  day!(2);
  day!(3);
  day!(4);
  day!(5);
}
