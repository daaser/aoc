const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> usize { partition().into_iter().next().unwrap_or_default() }

pub fn part2() -> usize { partition().into_iter().take(3).sum() }

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
  collector.sort_by(|a, b| b.cmp(a));
  collector
}

/*
=== DAY 1 ===
73211
213958
=============
 */
