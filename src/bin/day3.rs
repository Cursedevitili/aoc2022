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

static abc:&'static str = "abcdefghijklmnopqrstuvwxyz";
static ABC:&'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn calc_item_priority(item: &char) -> i32 {
    let mut how_many = 1;
    for i in abc.chars() {
        if i == *item {
            return how_many;
        }
        how_many += 1;
    }
    for i in ABC.chars() {
        if i == *item {
            return how_many;
        }
        how_many += 1;
    }

    how_many
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
        let mut count = 0;
        for i in self.comp1.chars() {
            for j in self.comp2.chars() {
                if i == j {
                    let prio = calc_item_priority(&i);
                    // println!("Found one: {}", i);
                    let item_to_add = WrongItem{
                        item_type: i.to_string(),
                        item_priority: prio
                    };

                    let mut vector = &mut self.wrong_items;
                    let mut add_here = true;
                    for item in &self.wrong_items {
                        if item.item_type == item_to_add.item_type {
                            add_here = false;
                        }
                    }
                    if add_here {
                        &mut self.wrong_items.push(item_to_add);
                    }
                }
            }
        }
    }
}

fn process_groups_three(group: Vec<Rucksack>) {
    for sack in group {

    }
}

fn main() {
    let input = shared::load_input("day3demo2.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut priority_sum = 0;
    let mut output = String::from("");

    let mut rucksackgroup: Vec<Rucksack> = Vec::new();
    for line in input.lines() {
        let mut rucksack = Rucksack::new(&line);
        rucksack.parse_wrong_items();
        if rucksackgroup.len() < 3 {
            rucksackgroup.push(rucksack.clone());
        } else {
            rucksackgroup.clear()
        }
        println!("{:?}", rucksack);
        let wrong_item_sum: i32 = rucksack.wrong_items.iter().map(|x| x.item_priority).sum();
        priority_sum += wrong_item_sum;
    }
    output.push_str(&format!("Sum of priorities: {}", priority_sum));
    shared::write_output("day3output.txt",&output);
}