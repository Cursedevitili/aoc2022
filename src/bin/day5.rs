use regex::Regex;

#[derive(Debug)]
struct  CargoShip {
    stacks: Vec<Vec<char>>
}

impl CargoShip {
    fn new() -> CargoShip {
        CargoShip {
            stacks: Vec::new()
        }
    }

    fn add_stack(&mut self) {
        self.stacks.push(Vec::new());
    }

    fn add_cargo(&mut self, what: char, which_stack: usize) {
        if what == ' '{
            return;
        }

        if which_stack > self.stacks.len() { panic!(); }
        self.stacks[which_stack].insert(0,what);
    }

    fn move_cargo(&mut self, order: Order) {
        let stack_len = self.stacks.len() as i32;
        let from = (order.from - 1) as usize;
        let to = (order.to - 1) as usize;
        let from_sane = if order.from <= stack_len { true } else { false };
        let to_sane = if order.to <= stack_len { true } else { false };
        let how_many = order.how_many;
        let mut helper_vec: Vec<char> = Vec::new();

        for _i in 0 ..how_many {
            match from_sane {
                true => {
                    let cargo_crate = self.stacks[from].pop();
                    match cargo_crate {
                        Some(x) => {
                            match to_sane {
                                true => { helper_vec.push(x); }
                                _ => { panic!(); }
                            }
                        }
                        None => { panic!(); }
                    }
                }
                _ => { panic!(); }
            }
        }

        for i in 0..helper_vec.len() {
            let removed_crate = helper_vec.pop().unwrap();
            self.stacks[to].push(removed_crate);
        }
    }
}

#[derive(Debug)]
struct Order {
    from: i32,
    to: i32,
    how_many:i32
}

impl Order {
    fn new(from:i32, to:i32, how_many:i32) -> Order {
        Order{
            from,
            to,
            how_many
        }
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day5.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut iter = input.lines();
    let mut cargo_len;
    let mut cargo_height = 0;
    loop {
        let mut line_opt = iter.next();
        match line_opt {
            Some(line) => {
                if line.contains("[") {
                    cargo_height += 1;
                    continue;
                }
                let asd= line.to_string().trim().replace("   ", " ");
                let asd: Vec<i32> = asd.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
                cargo_len = asd.len();
                break;
            }
            None => {}
        }
    }

    let mut ship = CargoShip::new();
    for k in 0..cargo_len {
        ship.add_stack();
    }

    let mut iter = input.lines();
    for _i in 0..cargo_height {
        let line = iter.next();
        match line {
            Some(x) => {
                let mut new_iter = x.chars();
                for j in 0..cargo_len{
                    let first = new_iter.next();
                    match first {
                        None => {break;}
                        Some(_) => {}
                    }
                    let cargo = new_iter.next().unwrap();
                    ship.add_cargo(cargo,j);
                    new_iter.next();
                    new_iter.next();
                }
            }
            None => {}
        }
    }
    iter.next();
    iter.next();
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut orders: Vec<Order> = Vec::new();
    loop {
        let mut line = iter.next();
        match line {
            None => {break}
            Some(x) => {
                let re = Regex::new(r"([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
                let caps = re.captures(x).unwrap();
                // println!("{:?}", caps);
                let from:i32 = caps[2].parse().unwrap();
                let to:i32 = caps[3].parse().unwrap();
                let how_many:i32 = caps[1].parse().unwrap();
                orders.push(Order::new(from, to, how_many))
            }
        }
    }

    // println!("{:?}", orders);
    for command in orders {
        ship.move_cargo(command);
    }
    println!("{:?}", ship);
    let mut secret_message = String::new();
    for stack in ship.stacks {
        secret_message.push(stack[stack.len()-1])
    }
    let output = format!("Secret message is: {}", secret_message);
    println!("Secret message {}", secret_message);
    aoc2022::shared::write_output("day5output.txt", &output);
}