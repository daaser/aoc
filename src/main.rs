mod day1;

fn main() -> eyre::Result<()> {
  println!(
    "=== DAY 1 ===\n{}\n{}\n=============",
    day1::most_calories(),
    day1::top_three_most_calories(),
  );

  Ok(())
}
