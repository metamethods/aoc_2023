use regex::Regex;

pub fn main(input: String) {
  let mut valid_games: Vec<u32> = vec![];

  let game_id_re = Regex::new(r"Game (\d+)").unwrap();
  let count_re = Regex::new(r",|;").unwrap();
  let valid_re = Regex::new(r"((\b[0-9]|10|11|12)\b red)|((\b[0-9]|10|11|12|13)\b green)|((\b[0-9]|10|11|12|13|14)\b blue)").unwrap();

  for line in input.lines() {
    let game_id = game_id_re.captures(&line)
      .unwrap()[1]
      .parse::<u32>().unwrap();
    let count: Vec<&str> = count_re.find_iter(&line)
      .map(|m| m.as_str()).collect();
    let valid: Vec<&str> = valid_re.find_iter(&line)
      .map(|m| m.as_str()).collect();

    if count.len() + 1 != valid.len() { continue; }

    valid_games.push(game_id);
  }

  println!("Part one answer {}", valid_games.iter().sum::<u32>());
}