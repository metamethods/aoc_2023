use regex::Regex;

pub fn main(input: String) {
  let mut power_sets: Vec<usize> = vec![];

  let re = Regex::new(r"(\d+) ([a-zA-Z]+)").unwrap();

  for l in input.lines() {
    let values: Vec<(&str, usize)> = re.captures_iter(&l).map(|caps| {
      let (_, [digit, name]) = caps.extract();
      (name, digit.parse::<usize>().unwrap())
    }).collect();

    let (mut r, mut b, mut g) = (0, 0, 0);

    for value in values {
      match value.0 {
        "red" => if value.1 > r { r = value.1 },
        "blue" => if value.1 > b { b = value.1 },
        "green" => if value.1 > g { g = value.1 }
        _ => panic!("Unknown value type!!")
      }
    }

    power_sets.push(r * b * g);
  }

  println!("Part 2 answer {}", power_sets.iter().sum::<usize>());
}