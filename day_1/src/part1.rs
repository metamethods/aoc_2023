pub fn main(input: String) {
  let mut numbers: Vec<u32> = vec![];

  for l in input.lines() {
    let values: Vec<u32> = l
      .chars()
      .map(|c| c.to_string().parse::<u32>())
      .filter(|v| v.is_ok())
      .map(|v| v.unwrap())
      .collect();
    numbers.push(
      format!("{}{}", 
        values.first().unwrap(), 
        values.last().unwrap()
      ).parse::<u32>().unwrap()
    );
  }

  println!("First part answer {}", numbers.iter().sum::<u32>());
}