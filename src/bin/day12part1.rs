#[derive(Debug, PartialEq)]
enum MoveDir {
    Left,
    Right,
    Up,
    Down,
    NotSet,
}

#[derive(Debug)]
struct PointCoord {
    current: (usize, usize),
    came_from_dir: MoveDir,
    finished: bool,
}

impl PointCoord {
    fn new(yx: (usize, usize), dir: MoveDir, finished: bool) -> PointCoord {
        let last = match dir {
            MoveDir::Left => MoveDir::Right,
            MoveDir::Right => MoveDir::Left,
            MoveDir::Up => MoveDir::Down,
            MoveDir::Down => MoveDir::Up,
            MoveDir::NotSet => MoveDir::NotSet
        };

        PointCoord {
            current: yx,
            came_from_dir: last,
            finished,
        }
    }

    fn get_cords(&self) -> (usize, usize) {
        (self.current.0 as usize, self.current.1 as usize)
    }
}

#[derive(Debug)]
enum MapPointType {
    Point(i32),
    MapStart(i32),
    MapFinish(i32),
}

impl MapPointType {
    fn get_val(&self) -> i32 {
        match self {
            MapPointType::Point(x) => { *x }
            MapPointType::MapStart(x) => { *x }
            MapPointType::MapFinish(x) => { *x }
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<MapPointType>>,
    h: usize,
    v: usize,
    points: Vec<PointCoord>,
    visited: Vec<(usize, usize)>,
    round: i32
}

impl Map {
    fn new(input: String) -> Map {
        let map_h = input.lines().count();
        let map_v: usize = input.lines().next().unwrap().chars().count();
        let mut map_vals: Vec<Vec<MapPointType>> = vec![];
        let mut count = 0;
        let mut points: Vec<PointCoord> = vec![];
        let mut visited: Vec<(usize, usize)> = vec![];
        let mut start_point = PointCoord {
            current: (0, 0),
            came_from_dir: MoveDir::Left,
            finished: false,
        };
        for line in input.lines() {
            map_vals.push(vec![]);
            for (j, c) in line.char_indices() {
                let val_to_push = Map::convert_char_to_map_val(c);
                if let MapPointType::MapStart(_) = val_to_push {
                    start_point = PointCoord::new((count, j), MoveDir::NotSet, false);
                    visited.push((count, j));
                }
                map_vals[count as usize].push(val_to_push);
            }
            count = count + 1;
        }
        points.push(start_point);

        Map {
            map: map_vals,
            h: map_h,
            v: map_v,
            points,
            visited: visited,
            round: 0
        }
    }

    fn convert_char_to_map_val(c: char) -> MapPointType {
        const KEY: &str = "SabcdefghijklmnopqrstuvwxyzE";
        let key_vec: Vec<char> = KEY.chars().collect();
        let pos: i32 = key_vec.iter().position(|&x| x == c).unwrap() as i32;

        match pos {
            0 => MapPointType::MapStart(pos),
            0..=26 => MapPointType::Point(pos),
            27 => MapPointType::MapFinish(pos),
            _ => { panic!() }
        }
    }

    fn get_possible_moves(&mut self) {
        let mut new_points: Vec<PointCoord> = vec![];
        for point in &self.points {
            let (y, x) = point.get_cords();
            let point_value = self.map[y][x].get_val();
            let limit_visited = 60;

            if x > 0 {
                let left_val = self.map[y][x - 1].get_val();
                if (left_val - point_value == 1 || left_val - point_value <= 0) &&
                    self.how_many_times_visited((y, x - 1)) < limit_visited {
                    let finish = if left_val == 27 { true } else { false };
                    let newpoint = PointCoord {
                        current: (y, x - 1),
                        came_from_dir: MoveDir::Left,
                        finished: finish,
                    };
                    self.visited.push((y, x - 1));
                    new_points.push(newpoint);
                }
            }

            if x < (self.v - 1) {
                let right_val = self.map[y][x + 1].get_val();
                if (right_val - point_value == 1 || right_val - point_value <= 0) &&
                    self.how_many_times_visited((y, x + 1)) < limit_visited {
                    let finish = if right_val == 27 { true } else { false };
                    let newpoint = PointCoord {
                        current: (y, x + 1),
                        came_from_dir: MoveDir::Right,
                        finished: finish,
                    };
                    self.visited.push((y, x + 1));
                    new_points.push(newpoint);
                }
            }

            if y > 0 {
                let up_val = self.map[y - 1][x].get_val();
                if (up_val - point_value == 1 || up_val - point_value <= 0) &&
                    self.how_many_times_visited((y - 1, x)) < limit_visited {
                    let finish = if up_val == 27 { true } else { false };
                    let newpoint = PointCoord {
                        current: (y - 1, x),
                        came_from_dir: MoveDir::Up,
                        finished: finish,
                    };
                    self.visited.push((y - 1, x));
                    new_points.push(newpoint);
                }
            }

            if y < (self.h - 1) {
                let down_val = self.map[y + 1][x].get_val();
                if (down_val - point_value == 1 || down_val - point_value <= 0) &&
                    self.how_many_times_visited((y + 1, x)) < limit_visited {
                    let finish = if down_val == 27 { true } else { false };
                    let newpoint = PointCoord {
                        current: (y + 1, x),
                        came_from_dir: MoveDir::Down,
                        finished: finish,
                    };
                    self.visited.push((y + 1, x));
                    new_points.push(newpoint);
                }
            }

        }
        self.points = new_points;
        self.round = self.round + 1;
    }

    fn how_many_times_visited(&self, point: (usize, usize)) -> i32 {
        let mut count = 0;
        for p in &self.visited {
            if *p == point {
                count = count + 1;
            }
        }
        count
    }

    fn print_map(&self) -> bool {
        let mut map_string = String::new();
        println!("Round: {}", self.round);
        for row in &self.map {
            for item in row {
                map_string.push_str(&item.get_val().to_string());
                map_string.push_str("\t");
            }
            map_string.push_str("\n");
        }
        for point in &self.points {
            map_string.push_str("Point");
            map_string.push_str("\t");
            map_string.push_str(&point.current.0.to_string());
            map_string.push_str("\t");
            map_string.push_str(&point.current.1.to_string());
            map_string.push_str("\t");
            if point.finished {
                map_string.push_str("Round: ");
                map_string.push_str(&self.round.to_string());
                map_string.push_str("\nFinished");
                println!("{}", map_string);
                return true
            }
            map_string.push_str("\n");

        }
        println!("{}", map_string);
        false
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day12.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    // println!("{}", input);

    let mut map = Map::new(input);
    map.print_map();

    // println!("{:?}", map);
    for i in 0 .. 500 {
        map.get_possible_moves();
        let ret = map.print_map();
        if ret {
            break;
        }
    }
}