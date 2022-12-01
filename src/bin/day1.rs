use std::process;
use aoc2022::shared;
use aoc2022::shared::write_output;

#[derive(Debug)]
struct Elf {
    number: i32,
    calories: i32,
}

fn main() {
    let input = shared::load_input("day1.txt");
    let input_txt = match input {
        Ok(contents) => {
            contents
        }
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };

    let mut elves = vec![Elf {number:1, calories:0}];
    let mut elf_number = 0;
    for line in input_txt.lines() {
        if line != "" {
            let snack:i32 = line.parse().unwrap();
            elves[elf_number].calories += snack;
        } else {
            let highest = elves.iter().map(|x| x.number).max().unwrap();
            let new_elf = Elf { number: highest + 1, calories: 0 };
            elves.push(new_elf);
            elf_number += 1;
        }
    }

    let most_calories = elves.iter().map(|x| x.calories).max().unwrap();
    let elf_with_most_calories = elves.iter().rfind(|&x| x.calories == most_calories).unwrap();

    let output = format!("elf number:{} calories: {}",elf_with_most_calories.number, elf_with_most_calories.calories);

    match write_output("day1output.txt", &output) {
        Ok(_) => {println!("Success")}
        Err(_) => {println!("Failed")}
    }
}