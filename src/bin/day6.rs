#[derive(Debug)]
struct DataStream {
    full: String,
    key_candidate: Vec<char>,
    message_candidate: Vec<char>,
    valid: bool,
    packet_start_at: i32,
    message_valid: bool,
    message_start_at: i32
}

impl DataStream {
    fn new(data: &str) -> DataStream {
        let mut iter = data.chars();
        let mut key_candidate: Vec<char> = Vec::new();
        for _ in  0..4 {
            key_candidate.push(iter.next().unwrap())
        }

        let mut iter = data.chars();
        let mut message_candidate: Vec<char> = Vec::new();
        for _ in  0..14 {
            message_candidate.push(iter.next().unwrap())
        }


        DataStream {
            full: data.to_string(),
            key_candidate,
            message_candidate,
            valid: false,
            packet_start_at: 4,
            message_valid: false,
            message_start_at: 14,
        }
    }

    fn packet_is_valid(&mut self) -> bool {
        let mut temp_key: Vec<char> = Vec::new();
        for i in 0..4 {
            if !temp_key.contains(&self.key_candidate[i]) {
                temp_key.push(self.key_candidate[i]);
            } else {
                return false
            }
        }

        true
        // let mut temp_key: Vec<char> = self.key_candidate.clone();
        // temp_key.dedup();
        //
        // if temp_key.len() == 4 {
        //     self.valid = true;
        //     return true;
        // } else {
        //     false
        // }
    }

    fn message_is_valid(&mut self) -> bool {
        let mut temp_message: Vec<char> = Vec::new();
        for i in 0..14 {
            if !temp_message.contains(&self.message_candidate[i]) {
                temp_message.push(self.message_candidate[i]);
            } else {
                return false
            }
        }

        true
    }

    fn move_key_candidate(&mut self) -> Result<(), ()> {
        let mut full_vec: Vec<char> = self.full.chars().collect();
        if self.packet_start_at < (self.full.len()) as i32 {
            self.key_candidate.push(full_vec[self.packet_start_at as usize]);
        } else {
            return Err(());
        }
        self.key_candidate.remove(0);
        self.packet_start_at += 1;
        Ok(())
    }

    fn move_message_candidate(&mut self) -> Result<(), ()> {
        let mut full_vec: Vec<char> = self.full.chars().collect();
        if self.message_start_at < (self.full.len()) as i32 {
            self.message_candidate.push(full_vec[self.message_start_at as usize]);
        } else {
            return Err(());
        }
        self.message_candidate.remove(0);
        self.message_start_at += 1;
        Ok(())
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day6.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut datastream = DataStream::new(&input);
    loop {
        let result = datastream.packet_is_valid();
        if result {
            break;
        }
        let  move_result = datastream.move_key_candidate();
        match move_result {
            Ok(_) => {}
            Err(_) => {panic!()}
        }
    }
    loop {
        let result = datastream.message_is_valid();
        if result {
            break;
        }
        let  move_result = datastream.move_message_candidate();
        match move_result {
            Ok(_) => {}
            Err(_) => {panic!()}
        }
    }

    let mut output = format!("Key start at: {}", datastream.packet_start_at);
    output.push_str(&format!("\nMessage start at: {}", datastream.message_start_at));
    println!("{}", output);
    aoc2022::shared::write_output("day6output.txt", &output).unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );
}