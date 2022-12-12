use std::io;

fn main() {
  let mut most_calories = 0;

  while let Some(calorie_list) = read_elf() {
    let total_calories = calorie_list.iter().sum();
    if total_calories > most_calories {
      most_calories = total_calories;
    }
  }

  println!("Most Calories an elf has are: {}", most_calories);
}

fn read_elf() -> Option<Vec<u32>> {
  let mut calories_list = Vec::new();

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
    calories_list.push(food_calories);
  }

  if calories_list.len() != 0 {
    Some(calories_list)
  } else {
    None
  }
}
