use aoc2022::shared;

#[derive(Debug, Clone)]
struct WrongItem {
    item_type: String,
    item_priority: i32,
}

#[derive(Debug, Clone)]
struct Rucksack{
    all_stuff: String,
    comp1: String,
    comp2: String,
    wrong_items: Vec<WrongItem>
}

fn calc_item_priority(item: char) {
    println!("{}", item.to_string())
}

impl Rucksack{
    fn new(contents: &str) -> Rucksack {
        let rucksack_length = contents.len();
        Rucksack {
            all_stuff: contents.to_string(),
            comp1: contents[0..rucksack_length / 2].to_string(),
            comp2: contents[(rucksack_length / 2)..rucksack_length].to_string(),
            wrong_items: vec![],
        }
    }

    fn parse_wrong_items(&mut self) {
        for (_, item1) in self.comp1.char_indices() {
            for (_, item2) in self.comp2.char_indices() {
                if item1 == item2 {
                    calc_item_priority(item1);
                    let item_to_add = WrongItem{
                        item_type: item1.to_string(),
                        item_priority: 3
                    };

                    let mut vector = &mut self.wrong_items;
                    vector.push(item_to_add);
                }
            }
        }
    }
}

fn main() {
    let input = shared::load_input("day3demo.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    for line in input.lines() {
        let mut rucksack = Rucksack::new(&line);
        rucksack.parse_wrong_items();
        // println!("{:?}", rucksack);
    }
}