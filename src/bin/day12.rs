#[derive(Debug)]
enum MoveDir {
    Left,
    Right,
    Up,
    Down,
    None
}

#[derive(Debug)]
struct PointCoord {
    current: (i32, i32),
    last: MoveDir,
    finished: bool
}

impl PointCoord {
    fn new(yx: (i32,i32), dir: MoveDir, goal_yx: &PointCoord) -> PointCoord {
        let last = match dir {
            Left =>  MoveDir::Right,
            Right =>  MoveDir::Left,
            Up =>  MoveDir::Down,
            Down =>  MoveDir::Up,
            MoveDir::None => MoveDir::None
        };
        let finished = if yx == goal_yx.get_cords() {
            true
        } else { false };

        PointCoord {
            current: yx,
            last,
            finished
        }
    }

    fn get_cords(&self) -> (i32, i32) {
        self.current
    }
    
    fn get_possible_moves(&self, map: Map){

    }
}

#[derive(Debug)]
enum MapPointType {
    Point(i32),
    MapStart(i32),
    MapFinish(i32)
}

struct Map {
    map: Vec<Vec<MapPointType>>,
    points: Vec<PointCoord>,
}

impl Map {
    fn new(input: String) -> Map {
        let map_h = input.lines().count();
        let map_v: usize = input.lines().next().unwrap().chars().count();
        let mut map_vals: Vec<Vec<MapPointType>> = vec![];
        let mut count = 0;
        let mut points: Vec<PointCoord> = vec![];
        let mut start_point;
        for line in input.lines() {
            map_vals.push(vec![]);
            for (j, c) in line.char_indices() {
                let val_to_push = Map::convert_char_to_map_val(c);
                if let MapPointType::MapStart(_) = val_to_push {
                    start_point = PointCoord::new((count, j as i32), MoveDir::None, (0,0))
                }
                map_vals[count].push(val_to_push);
            }
            count = count + 1;
        }

        Map {
            map: map_vals,
            points
        }
    }

    fn convert_char_to_map_val (c: char) -> MapPointType {
        const KEY: &str = "SabcdefghijklmnopqrstuvwxyzE";
        let key_vec: Vec<char> = KEY.chars().collect();
        let pos: i32 = key_vec.iter().position(|&x| x == c).unwrap() as i32;

        match pos {
            0 => MapPointType::MapStart(pos),
            0 ..=26 =>MapPointType::Point(pos),
            27 => MapPointType::MapFinish(pos),
            _ => {panic!()} }
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day12demo.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    println!("{}",input);

}