struct Grid {
    map_len_h: i32,
    map_len_v: i32,
    head_pos: (i32, i32),
    tail1_pos: (i32, i32),
    tail2_pos: (i32, i32),
    tail3_pos: (i32, i32),
    tail4_pos: (i32, i32),
    tail5_pos: (i32, i32),
    tail6_pos: (i32, i32),
    tail7_pos: (i32, i32),
    tail8_pos: (i32, i32),
    tail9_pos: (i32, i32),
    tail_positions: Vec<(i32, i32)>,
}

#[derive(Debug)]
enum Dir {
    UP(i32),
    DOWN(i32),
    LEFT(i32),
    RIGHT(i32),
}

impl Dir {
    fn build(dir: char, how_many: i32) -> Result<Dir, ()> {
        match dir {
            'R' => Ok(Dir::RIGHT(how_many)),
            'L' => Ok(Dir::LEFT(how_many)),
            'U' => Ok(Dir::UP(how_many)),
            'D' => Ok(Dir::DOWN(how_many)),
            _ => { Err(()) }
        }
    }
}

impl Grid {
    fn new() -> Grid {
        Grid {
            map_len_h: 15,
            map_len_v: 15,
            head_pos: (4, 0),
            tail1_pos: (4, 0),
            tail2_pos: (4, 0),
            tail3_pos: (4, 0),
            tail4_pos: (4, 0),
            tail5_pos: (4, 0),
            tail6_pos: (4, 0),
            tail7_pos: (4, 0),
            tail8_pos: (4, 0),
            tail9_pos: (4, 0),
            tail_positions: vec![(5, 0)],
        }
    }

    fn calc_len(&self) -> i32 {
        if self.head_pos == self.tail1_pos {
            return 0;
        }

        if self.head_pos.0 == self.tail1_pos.0 {
            return (self.head_pos.1 - self.tail1_pos.1).abs() - 1;
        } else if self.head_pos.1 == self.tail1_pos.1 {
            return (self.head_pos.0 - self.tail1_pos.0).abs() - 1;
        }

        let y_diff = (self.head_pos.0 - self.tail1_pos.0).abs();
        let x_diff = (self.head_pos.1 - self.tail1_pos.1).abs();
        x_diff + y_diff - 2
    }

    fn move_head(&mut self, dir: Dir) {
        match dir {
            Dir::UP(x) => {
                for _ in 0..x {
                    let head_last_pos = self.head_pos;
                    self.head_pos = (self.head_pos.0 - 1, self.head_pos.1);
                    self.move_tail(head_last_pos);
                    self.print_state();
                }
            }
            Dir::DOWN(x) => {
                for _ in 0..x {
                    let head_last_pos = self.head_pos;
                    self.head_pos = (self.head_pos.0 + 1, self.head_pos.1);
                    self.move_tail(head_last_pos);
                    self.print_state();
                }
            }
            Dir::LEFT(x) => {
                for _ in 0..x {
                    let head_last_pos = self.head_pos;
                    self.head_pos = (self.head_pos.0, self.head_pos.1 - 1);
                    self.move_tail(head_last_pos);
                    self.print_state();
                }
            }
            Dir::RIGHT(x) => {
                for _ in 0..x {
                    let head_last_pos = self.head_pos;
                    self.head_pos = (self.head_pos.0, self.head_pos.1 + 1);
                    self.move_tail(head_last_pos);
                    self.print_state();
                }
            }
        }
    }

    fn move_tail(&mut self, head_last_pos: (i32, i32)) {
        let h_t_diff = self.calc_len();
        if h_t_diff == 1 {
            self.tail1_pos = head_last_pos;
            if !self.tail_positions.contains(&self.tail1_pos) {
                self.tail_positions.push(self.tail1_pos);
            }
        }
    }

    fn print_state(&self) {
        let mut map: Vec<Vec<char>> = vec![];
        for x in 0..self.map_len_v {
            map.push(vec![]);
        }
        for i in 0..self.map_len_h {
            for j in 0..self.map_len_v {
                if self.head_pos == (i, j) {
                    map[i as usize].push('H');
                } else if self.tail1_pos == (i, j) {
                    map[i as usize].push('T');
                } else if (i, j) == (4, 0) {
                    map[i as usize].push('S');
                } else {
                    map[i as usize].push('.')
                }
            }
        }

        let mut state = String::new();
        for i in map {
            for j in i {
                state.push(j);
            }
            state.push('\n');
        }
        println!("{}", state);
    }

    fn print_tail_visited(&mut self) {
        let mut map: Vec<Vec<char>> = vec![];
        for x in 0..self.map_len_v {
            map.push(vec![]);
        }
        for i in 0..self.map_len_h {
            for j in 0..self.map_len_v {
                if self.tail_positions.contains(&(i, j)) {
                    map[i as usize].push('#')
                } else if (i, j) == (4, 0) {
                    map[i as usize].push('S');
                } else {
                    map[i as usize].push('.')
                }
            }
        }

        let mut final_state = String::new();
        for i in map {
            for j in i {
                final_state.push(j);
            }
            final_state.push('\n');
        }
        println!("{}", final_state);
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day9.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );
    let mut commands: Vec<Dir> = vec![];
    for line in input.lines() {
        let mut iter = line.chars();
        let dir_char = iter.next().unwrap();
        iter.next();
        let many: i32 = iter.next()
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        let last_possible = iter.next();
        match last_possible {
            None => { commands.push(Dir::build(dir_char, many).unwrap_or_else(|_| panic!())) }
            Some(x) => {
                let mut num_string = many.to_string();
                num_string.push(x);
                commands.push(Dir::build(dir_char, num_string.parse().unwrap()).unwrap_or_else(|_| panic!()));
            }
        }
    }

    let mut grid = Grid::new();
    for cmd in commands {
        println!("{:?}", cmd);
        grid.move_head(cmd);
        // grid.print_state();
        println!();
    }
    grid.print_tail_visited();
    let tails_pos_len = grid.tail_positions.len();
    let mut out1 = String::from("Tails positions: ");
    out1.push_str(&tails_pos_len.to_string());
    println!("{}",out1);
}