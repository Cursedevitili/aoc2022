struct Grid {
    map_len_h: i32,
    map_len_v: i32,
    head_pos: (i32, i32),
    tail_pos: (i32, i32)
}

enum Dir {
    UP(u32),
    DOWN(u32),
    LEFT(u32),
    RIGHT(u32)
}

impl Dir {
    fn parse_from_str(dir: &str, how_many: u32) -> Result<Dir, ()>{
        match dir {
            "R" => Ok(Dir::RIGHT(how_many)),
            "L" => Ok(Dir::LEFT(how_many)),
            "U" => Ok(Dir::UP(how_many)),
            "D" => Ok(Dir::DOWN(how_many)),
            &_ => {Err(())}
        }
    }
}

impl Grid {
    fn new () -> Grid {
        Grid{
            map_len_h: 6,
            map_len_v: 6,
            head_pos: (5,0),
            tail_pos: (5,0)
        }
    }

    fn calc_len(&self) -> i32 {
        if self.head_pos == self.tail_pos {
            return 0;
        }

        if self.head_pos.0 == self.tail_pos.0 {
            return (self.head_pos.1 - self.tail_pos.1).abs()-1
        }
        else if self.head_pos.1 == self.tail_pos.1 {
            return (self.head_pos.0 - self.tail_pos.0).abs()-1
        }

        let y_diff = (self.head_pos.0 - self.tail_pos.0).abs();
        let x_diff = (self.head_pos.1 - self.tail_pos.1).abs();
        x_diff + y_diff - 2
    }

    fn print_state(self){
        let mut map:Vec<Vec<char>> = vec![];
        for x in 0.. self.map_len_v {
            map.push(vec![]);
        }
        for i in 0 .. self.map_len_h {
            for j in 0 .. self.map_len_v {
                if self.head_pos == (i,j) {
                    map[i as usize].push('H');
                } else if self.tail_pos == (i,j) {
                    map[i as usize].push('T');
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
}

fn main() {
    let mut grid = Grid::new();
    grid.print_state();
}