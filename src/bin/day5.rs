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
        if which_stack > self.stacks.len() { panic!(); }
        self.stacks[which_stack-1].push(what);
    }

    fn move_cargo(&mut self, order: Order) {
        let stack_len = self.stacks.len() as i32;
        let from = (order.from - 1) as usize;
        let to = (order.to - 1) as usize;
        let from_sane = if order.from < stack_len { true } else { false };
        let to_sane = if order.to < stack_len { true } else { false };
        let how_many = order.how_many;

        for _i in 0 ..how_many {
            match from_sane {
                true => {
                    let cargo_crate = self.stacks[from].pop();
                    match cargo_crate {
                        Some(x) => {
                            match to_sane {
                                true => { self.stacks[to].push(x) }
                                _ => { panic!(); }
                            }
                        }
                        None => { panic!(); }
                    }
                }
                _ => { panic!(); }
            }
        }
    }
}

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
    let input = aoc2022::shared::load_input("day5demo.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut iter = input.lines();
    let mut cargo_len;
    loop {
        let mut line_opt = iter.next();
        match line_opt {
            Some(line) => {
                if line.contains("[") {
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

    println!("cargo len:{cargo_len}")


}