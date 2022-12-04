#[derive(Debug)]
struct ChorePair {
    Chore1: String,
    Chore2: String,
    ContainsOther: Option<bool>
}

impl ChorePair {
    fn new(inp: &str) -> ChorePair {
        let mut  line_chars = inp.chars();
        let mut start_string1 = String::new();
        loop {
            let char = line_chars.next().unwrap().to_string();
            if char != "-" {
                start_string1.push_str(&char);
            } else {
                break;
            }
        }

        let mut end_string1 = String::new();
        loop {
            let char = line_chars.next().unwrap().to_string();
            if char != "," {
                end_string1.push_str(&char);
            } else {
                break;
            }
        }

        let mut start_string2 = String::new();
        loop {
            let char = line_chars.next().unwrap().to_string();
            if char != "-" {
                start_string2.push_str(&char);
            } else {
                break;
            }
        }

        let mut end_string2 = String::new();
        loop {
            let char = line_chars.next();
            match char {
                Some(_) =>{
                    end_string2.push_str(&char.unwrap().to_string());
                },
                None => {break;}
            }
        }

        let start1: i32 = start_string1.parse().unwrap();
        let end1: i32 = end_string1.parse().unwrap();
        let mut chore1:String = String::new();
        if start1 != end1 {
            for i in start1..end1+1 {
                chore1.push_str(&"b");
                chore1.push_str(&i.to_string());
                chore1.push_str(&"b");
            }
        } else {
            chore1.push_str(&"b");
            chore1.push_str(&start1.to_string());
            chore1.push_str(&"b");
        }

        let start2: i32 = start_string2.parse().unwrap();
        let end2: i32 = end_string2.parse().unwrap();
        let mut chore2:String = String::new();
        if start2 != end2 {
            for i in start2..end2+1 {
                chore2.push_str(&"b");
                chore2.push_str(&i.to_string());
                chore2.push_str(&"b");
            }
        } else {
            chore2.push_str(&"b");
            chore2.push_str(&start2.to_string());
            chore2.push_str(&"b");
        }

        ChorePair {
            Chore1: chore1,
            Chore2: chore2,
            ContainsOther: None
        }
    }

    fn process_contains(&mut self) -> bool {
        if self.Chore1.contains(&self.Chore2) {
            self.ContainsOther = Some(true);
            return true;
        }

        if self.Chore2.contains(&self.Chore1) {
            self.ContainsOther = Some(true);
            return true;
        }

        false
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day4.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut sum = 0;
    for line in input.lines() {
        let mut pair = ChorePair::new(&line);
        let  result = pair.process_contains();
        println!("{:?}",pair);
        if result {
            sum += 1;
        }
    }

    let output = format!("Chore pairs fully containing other one:{}", sum);
    aoc2022::shared::write_output("day4output.txt",&output);
}