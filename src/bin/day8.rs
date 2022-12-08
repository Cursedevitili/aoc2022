use std::collections::hash_map::Values;
use regex::internal::Input;

#[derive(Debug)]
struct Forest {
    contents: Vec<Vec<i32>>
}

impl Forest {
    fn new(input: &str) -> Forest {
        let mut root:Vec<Vec<i32>> = vec![];
        for row in input.lines() {
            let forest_row:Vec<i32> = row.chars().map(|x| x.to_string().parse::<i32>().unwrap() ).collect();
            root.push(forest_row);
        }
        Forest {
            contents: root
        }
    }
    
    fn get_not_visible(&self) -> i32{
        for i in 0..self.contents.len() {
            for j in 0..self.contents[i].len() {
                print!("{}", self.contents[i][j]);
            }
        }
        
        0
    }

}

fn main() {
    let input = aoc2022::shared::load_input("day8demo.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );
    
    let mut forest = Forest::new(&input);
    print!("{:?}", forest);
    let result = forest.get_not_visible();
}