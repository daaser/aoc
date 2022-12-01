const INPUT: &'static str = include_str!("input.txt");

pub fn most_calories() -> usize {
  let input = partition();
  input.into_iter().last().unwrap_or_default()
}

pub fn top_three_most_calories() -> usize {
  let input = partition();
  let i_len = input.len();
  input.into_iter().skip(i_len - 3).sum()
}

fn partition() -> Vec<usize> {
  let mut count = 0usize;
  let mut collector = vec![];
  for line in INPUT.lines() {
    if line.is_empty() {
      collector.push(count);
      count = 0;
      continue;
    }
    count += line.parse::<usize>().unwrap();
  }
  collector.sort();
  collector
}