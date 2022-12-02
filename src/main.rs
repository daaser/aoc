mod day1;
mod day2;

fn main() -> eyre::Result<()> {
  println!(
    "=== DAY 1 ===\n{}\n{}\n=============",
    day1::most_calories(),
    day1::top_three_most_calories(),
  );

  println!(
    "=== DAY 2 ===\n{}\n{}\n=============",
    day2::perfect_strategy(),
    day2::perfecter_strategy(),
  );

  Ok(())
}
