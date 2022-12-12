use std::io;

fn main() {
  let mut most_calories = 0;
  let mut second_most_calories = 0;
  let mut third_most_calories = 0;

  loop {
    let current_elf_calories = read_elf();
  }

  println!("Most Calories an elf has are: {}", most_calories);
}

fn read_elf() -> u32 {
  let mut total_calories: u32 = 0;

  loop {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
      Ok(0) => break,
      Ok(_) => {},
      Err(e) => panic!("Error while reading stdin: {}", e)
    }

    if line == "\n" {
      break;
    }

    let food_calories = line.trim().parse::<u32>().unwrap();
    total_calories += food_calories;
  }

  total_calories
}
