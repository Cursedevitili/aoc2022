struct Grid {
    map_len_uni: i32,
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
    tail1_positions: Vec<(i32, i32)>,
    tail2_positions: Vec<(i32, i32)>,
    tail3_positions: Vec<(i32, i32)>,
    tail4_positions: Vec<(i32, i32)>,
    tail5_positions: Vec<(i32, i32)>,
    tail6_positions: Vec<(i32, i32)>,
    tail7_positions: Vec<(i32, i32)>,
    tail8_positions: Vec<(i32, i32)>,
    tail9_positions: Vec<(i32, i32)>,
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

enum KnotMoveDir{
    NONE,
    UP,
    UpRight,
    UpLeft,
    DOWN,
    DownRight,
    DownLeft,
    LEFT,
    RIGHT,
}

impl KnotMoveDir {
    fn get(head:(i32, i32), tail:(i32, i32)) -> KnotMoveDir {
        let diff = (head.0 - tail.0, head.1 - tail.1);
        if (diff.0 == -1 || diff.0 == 0 || diff.0 == 1) &&
            (diff.1 == -1 || diff.1 == 0 || diff.1 == 1) {
            return KnotMoveDir::NONE;
        }

        let dir = match diff {
            (-1, 2) => KnotMoveDir::UpRight,
            (-2, 1) => KnotMoveDir::UpRight,
            (-2, 2) => KnotMoveDir::UpRight,
            (-1,-2) => KnotMoveDir::UpLeft,
            (-2,-1) => KnotMoveDir::UpLeft,
            (-2,-2) => KnotMoveDir::UpLeft,
            (1, 2) => KnotMoveDir::DownRight,
            (2, 1) => KnotMoveDir::DownRight,
            (2, 2) => KnotMoveDir::DownRight,
            (1, -2) => KnotMoveDir::DownLeft,
            (2, -1) => KnotMoveDir::DownLeft,
            (2, -2) => KnotMoveDir::DownLeft,
            // Easy ones
            (0, 2) => KnotMoveDir::RIGHT,
            (0, -2) => KnotMoveDir::LEFT,
            (-2, 0) => KnotMoveDir::UP,
            (2, 0) => KnotMoveDir::DOWN,
            (_, _) => {panic!()} };

        dir
    }
}

impl Grid {
    fn new() -> Grid {
        Grid {
            map_len_uni: 100,
            map_len_h: 30,
            map_len_v: 30,
            head_pos: (0, 0),
            tail1_pos: (0, 0),
            tail2_pos: (0, 0),
            tail3_pos: (0, 0),
            tail4_pos: (0, 0),
            tail5_pos: (0, 0),
            tail6_pos: (0, 0),
            tail7_pos: (0, 0),
            tail8_pos: (0, 0),
            tail9_pos: (0, 0),
            tail1_positions: vec![(0, 0)],
            tail2_positions: vec![(0, 0)],
            tail3_positions: vec![(0, 0)],
            tail4_positions: vec![(0, 0)],
            tail5_positions: vec![(0, 0)],
            tail6_positions: vec![(0, 0)],
            tail7_positions: vec![(0, 0)],
            tail8_positions: vec![(0, 0)],
            tail9_positions: vec![(0, 0)],
        }
    }

    fn calc_len(&self, knot_num: i32) -> i32 {
        let mut head;
        let mut tail;
        match knot_num  {
            1 => {
                head = &self.head_pos;
                tail = &self.tail1_pos;
            }
            2 => {
                head = &self.tail1_pos;
                tail = &self.tail2_pos;
            }
            3 => {
                head = &self.tail2_pos;
                tail = &self.tail3_pos;
            }
            4 => {
                head = &self.tail3_pos;
                tail = &self.tail4_pos;
            }
            5 => {
                head = &self.tail4_pos;
                tail = &self.tail5_pos;
            }
            6 => {
                head = &self.tail5_pos;
                tail = &self.tail6_pos;
            }
            7 => {
                head = &self.tail6_pos;
                tail = &self.tail7_pos;
            }
            8 => {
                head = &self.tail7_pos;
                tail = &self.tail8_pos;
            }
            9 => {
                head = &self.tail8_pos;
                tail = &self.tail9_pos;
            }

            _ => { panic!()} }


        if head == tail {
            return 0;
        }

        if head.0 == tail.0 {
            return (head.1 - tail.1).abs() - 1;
        } else if head.1 == tail.1 {
            return (head.0 - tail.0).abs() - 1;
        }

        let y_diff = (head.0 - tail.0).abs();
        let x_diff = (head.1 - tail.1).abs();
        x_diff + y_diff - 2
    }


    fn move_head(&mut self, dir: Dir) {
        match dir {
            Dir::UP(x) => {
                for _ in 0..x {
                    let head_last_pos = self.head_pos;
                    self.head_pos = (self.head_pos.0 - 1, self.head_pos.1);
                    self.move_tail_head(head_last_pos, 1);
                    self.move_tail_rest();
                    self.print_state();
                }
            }
            Dir::DOWN(x) => {
                for _ in 0..x {
                    let head_last_pos = self.head_pos;
                    self.head_pos = (self.head_pos.0 + 1, self.head_pos.1);
                    self.move_tail_head(head_last_pos, 1);
                    self.move_tail_rest();
                    self.print_state();
                }
            }
            Dir::LEFT(x) => {
                for _ in 0..x {
                    let head_last_pos = self.head_pos;
                    self.head_pos = (self.head_pos.0, self.head_pos.1 - 1);
                    self.move_tail_head(head_last_pos, 1);
                    self.move_tail_rest();
                    self.print_state();
                }
            }
            Dir::RIGHT(x) => {
                for _ in 0..x {
                    let head_last_pos = self.head_pos;
                    self.head_pos = (self.head_pos.0, self.head_pos.1 + 1);
                    self.move_tail_head(head_last_pos, 1);
                    self.move_tail_rest();
                    self.print_state();
                }
            }
        }
    }

    fn move_tail_head(&mut self, head_last_pos: (i32, i32), knot_num: i32) {
        let h_t_diff = self.calc_len(1);

        let mut head= &mut self.head_pos;
        let mut tail =&mut self.tail1_pos;
        let mut tail_movement = &mut self.tail1_positions;

        if h_t_diff == 1 {
            *tail = head_last_pos;
            if !tail_movement.contains(&tail) {
                tail_movement.push(self.tail1_pos);
            }
        }
    }

    fn move_tail_rest(&mut self) {
        let mut tail_head_pos = self.tail1_pos;
        for i in 2..10 {
            let mut knot = match i {
                2 => &mut self.tail2_pos,
                3 => &mut self.tail3_pos,
                4 => &mut self.tail4_pos,
                5 => &mut self.tail5_pos,
                6 => &mut self.tail6_pos,
                7 => &mut self.tail7_pos,
                8 => &mut self.tail8_pos,
                9 => &mut self.tail9_pos,
                _ => {panic!()} };


            let dir = KnotMoveDir::get(tail_head_pos, *knot);
            match dir {
                KnotMoveDir::NONE => {}
                KnotMoveDir::UP => {knot.0 = knot.0-1;}
                KnotMoveDir::UpRight => {
                    knot.0 = knot.0-1;
                    knot.1 = knot.1+1;
                }
                KnotMoveDir::UpLeft => {
                    knot.0 = knot.0-1;
                    knot.1 = knot.1-1;
                }
                KnotMoveDir::DOWN => {knot.0 = knot.0+1;}
                KnotMoveDir::DownRight => {
                    knot.0 = knot.0+1;
                    knot.1 = knot.1+1;
                }
                KnotMoveDir::DownLeft => {
                    knot.0 = knot.0+1;
                    knot.1 = knot.1-1;
                }
                KnotMoveDir::LEFT => {knot.1 = knot.1-1;}
                KnotMoveDir::RIGHT => {knot.1 = knot.1+1;}
            }
            tail_head_pos = *knot;

            if i == 9 {
                if !self.tail9_positions.contains(knot){
                    self.tail9_positions.push(*knot);
                }
            }
        }
    }

    fn print_state(&self) {
        let mut map: Vec<Vec<char>> = vec![];
        for _ in 0..self.map_len_uni {
            map.push(vec![]);
        }
        for x in &mut map {
            for _ in 0..self.map_len_uni {
                x.push('X');
            }
        }


        for i in -self.map_len_uni/2..self.map_len_uni/2 {
            for j in -self.map_len_uni/2..self.map_len_uni/2 {
                if self.head_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = 'H';
                } else if self.tail1_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '1';
                } else if self.tail2_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '2';
                } else if self.tail3_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '3';
                } else if self.tail4_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '4';
                }else if self.tail5_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '5';
                }else if self.tail6_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '6';
                }else if self.tail7_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '7';
                }else if self.tail8_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '8';
                }else if self.tail9_pos == (i, j) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '9';
                }
                else if (i, j) == (0, 0) {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = 'S';
                } else {
                    map[(i + self.map_len_uni / 2) as usize][(j + self.map_len_uni / 2) as usize] = '.'
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
        for i in -50..50 {
            for j in -50..50 {
                if self.tail1_positions.contains(&(i, j)) {
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
    grid.print_state();
    println!();
    for cmd in commands {
        println!("{:?}", cmd);
        grid.move_head(cmd);
        // grid.print_state();
        println!();
    }
    // grid.print_tail_visited();
    let tails_pos_len = grid.tail1_positions.len();
    let mut out1 = String::from("Tails positions: ");
    out1.push_str(&tails_pos_len.to_string());
    let mut out2 = String::from("\nTail 9 positions: ");
    out2.push_str(&grid.tail9_positions.len().to_string());
    out1.push_str(&out2);
    println!("{}",out1);
    println!("{}",out2);
    aoc2022::shared::write_output("day9output.txt", &out1);
}