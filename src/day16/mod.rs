use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Valve {
  name: String,
  flow_rate: usize,
  neighbors: Vec<String>,
}

fn parse() -> HashMap<String, Valve> {
  let _input = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
  let mut valves = HashMap::new();
  for line in _input.lines() {
    let words = line.split_whitespace().collect::<Vec<_>>();
    let name = words[1].to_string();
    let flow_rate: usize;
    text_io::scan!(words[4].bytes() => "rate={};", flow_rate);
    let neighbors = words[9..].iter().map(|word| word.trim_matches(',').to_string()).collect();
    valves.insert(name.clone(), Valve { name, flow_rate, neighbors });
  }
  valves
}

// fn optimal_route(valves: HashMap<String, Valve>) -> usize {
//   let mut choices = HashMap::new();
//   choices.insert((0, "AA".to_string()), )
//   for i in 0..30 {
//     let mut new_choices = HashMap::new();
//     for choice in &choices {}
//     // for n in curr.neighbors {
//     //   let valve = &valves[&n];
//     //   let flow_rate = valve.flow_rate;
//     //   // new_choices.insert()
//     // }
//     //
//     choices = new_choices;
//   }
//   0
// }

pub fn part_one() -> usize {
  let valves = parse();
  // optimal_route(valves);
  0
}

pub fn part_two() -> usize {
  0
}

/*
1728 2304
 */
