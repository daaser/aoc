mod day1;

fn main() -> eyre::Result<()> {
  println!("{:?}", day1::most_calories());
  println!("{:?}", day1::top_three_most_calories());

  Ok(())
}
