mod day1;
mod day2;
mod day3;

fn main() {
  println!(
    "=== DAY 1 ====\n{}\n{}\n==============\n",
    day1::part1(),
    day1::part2(),
  );

  println!(
    "=== DAY 2 ====\n{}\n{}\n==============\n",
    day2::part1(),
    day2::part2(),
  );

  println!(
    "=== DAY 3 ====\n{}\n{}\n==============\n",
    day3::part1(),
    day3::part2(),
  );
}
