const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Rectangle {
  x_max: isize,
  y_max: isize,
  x_min: isize,
  y_min: isize,
}

impl Rectangle {
  fn intersects(&self, other: &Self) -> bool {
    let dx = self.x_max.min(other.x_max) - self.x_min.max(other.x_min);
    let dy = self.y_max.min(other.y_max) - self.y_min.max(other.y_min);
    dx >= 0 && dy >= 0
  }

  fn overlap(&self, other: &Self) -> (isize, isize) {
    (self.x_max.min(other.x_max), self.x_min.max(other.x_min))
    // let dy = self.y_max.min(other.y_max) - self.y_min.max(other.y_min);
    // dx
  }
}

// | x1 − x2 | + | y1 − y2 |
fn no_beacons(y: isize) -> isize {
  let _input = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
  let mut beacons_y = vec![];
  let mut rectangles = vec![];
  let mut min_x = isize::MAX;
  let mut max_x = isize::MIN;
  for line in _input.lines() {
    let (x1, y1, x2, y2): (isize, isize, isize, isize);
    text_io::scan!(line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", x1, y1, x2, y2);
    let m_dist = (x1 - x2).abs() + (y1 - y2).abs();

    if y2 == y {
      beacons_y.push((x2, y2));
    }

    min_x = min_x.min(x1.min(x2));
    max_x = max_x.max(x1.max(x2));

    println!("x1: {x1:2}, y1: {y1:2}, x2: {x2:2}, y2: {y2:2}, m_dist: {m_dist}");

    let x_max = x1 + m_dist;
    let y_max = y1 + m_dist;
    let x_min = x1 - m_dist;
    let y_min = y1 - m_dist;

    let rec = Rectangle { x_max, y_max, x_min, y_min };
    println!("{:?}\n", rec);
    rectangles.push(rec);
  }

  let max_y = dbg!(beacons_y.iter().map(|(_, y)| y).max().unwrap());
  let min_y = dbg!(beacons_y.iter().map(|(_, y)| y).min().unwrap());
  let y_rec = Rectangle { x_min: min_x, x_max: max_x, y_min: *min_y, y_max: *max_y };
  let mut candidates = vec![];
  for rec in rectangles.iter() {
    if rec.intersects(&y_rec) {
      println!("{:?} overlaps with {:?}", rec, y_rec);
      candidates.push(rec);
    }
  }

  let max = dbg!(candidates.iter().map(|r| r.overlap(&y_rec).0).max().unwrap());
  let min = dbg!(candidates.iter().map(|r| r.overlap(&y_rec).1).min().unwrap());
  max - min - beacons_y.len() as isize
  // candidates.iter().map((|r| r.area(&y_rec))).sum()
}

pub fn part_one() -> isize {
  no_beacons(10)
}

pub fn part_two() -> usize {
  0
}

/*
┏━━━━ DAY 15 ━━━━┓
┃5394423         ┃
┃11840879211051  ┃
┗━━━━━━━━━━━━━━━━┛
 */
