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


fn process_groups_three(group: Vec<Rucksack>) -> char {
    let sack1contents = group[0].all_stuff.chars();
    let sack2contents = &group[1].all_stuff;
    let sack3contents = &group[2].all_stuff;
    for item in sack1contents {
        let mut item_in_2: bool = false;
        let mut item_in_3: bool = false;
        if sack2contents.contains(&item.to_string()) {
            item_in_2 = true;
        }
        if sack3contents.contains(&item.to_string()) {
            item_in_3 =true;
        }

        if item_in_2 && item_in_3 {
            return item;
        }
    }

    'å'
}

fn main() {
    let input = shared::load_input("day3.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut priority_sum = 0;
    let mut badges_value_sum = 0;
    let mut output = String::from("");

    let mut rucksackgroup: Vec<Rucksack> = Vec::new();
    for line in input.lines() {
        let mut rucksack = Rucksack::new(&line);
        rucksack.parse_wrong_items();
        if rucksackgroup.len() < 3 {
            rucksackgroup.push(rucksack.clone());
        }
        if rucksackgroup.len() == 3 {
            let shared_item = process_groups_three(rucksackgroup.clone());
            let value = calc_item_priority(&shared_item);
            badges_value_sum += value;
            println!("shared_prio:{}", shared_item);
            rucksackgroup.clear()
        }
        println!("{:?}", rucksack);
        let wrong_item_sum: i32 = rucksack.wrong_items.iter().map(|x| x.item_priority).sum();
        priority_sum += wrong_item_sum;
    }
    output.push_str(&format!("Sum of priorities: {}", priority_sum));
    output.push_str(&format!("\nSum of badge value: {}", badges_value_sum));
    shared::write_output("day3output.txt",&output);
}