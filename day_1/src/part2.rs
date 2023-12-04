pub fn main(input: String) {
  let mut numbers: Vec<u32> = vec![];

  for l in input.lines() {
    let l = l
      .replace("one", "o1e")
      .replace("two", "t2o")
      .replace("three", "thr3e")
      .replace("four", "fo4r")
      .replace("five", "fi5e")
      .replace("six", "s6x")
      .replace("seven", "se7en")
      .replace("eight", "ei8ht")
      .replace("nine", "ni9e");

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

  println!("Second part answer {}", numbers.iter().sum::<u32>());
}