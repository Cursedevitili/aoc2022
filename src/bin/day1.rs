use std::process;
use aoc2022::shared;
use aoc2022::shared::write_output;

#[derive(Debug, Copy, Clone)]
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

    let mut top_3_elves = vec![];
    for _i in 0..3 {
        let most_calories = elves.iter().map(|x| x.calories).max().unwrap();
        let elf_with_most_calories = elves.iter().rfind(|&x| x.calories == most_calories).unwrap().clone();
        let elf_to_remove_number = elves.iter().position(|x| x.number == elf_with_most_calories.number).unwrap();
        top_3_elves.push(elf_with_most_calories);
        elves.remove(elf_to_remove_number);
    }

    let mut output = String::new();
    for elf in &top_3_elves {
        output.push_str(&format!("elf number:{} calories: {}\n", elf.number, elf.calories))
    }
    let sum: i32 = top_3_elves.iter().map(|x| x.calories).sum();
    output.push_str(&format!("Sum: {sum}"));

    match write_output("day1output.txt", &output) {
        Ok(_) => {println!("Success")}
        Err(_) => {println!("Failed")}
    }
}