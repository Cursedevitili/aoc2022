use regex::Regex;
use crate::NMove::Stop;

#[derive(Debug, Clone)]
struct Cmd {
    x: i32,
    y: i32,
}

impl Cmd {
    fn get_yx(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }
}

#[derive(PartialEq)]
enum MapObj {
    Air,
    Rock,
    Sand,
    Source,
}

#[derive(Clone, Copy)]
enum NMove {
    New(usize, usize),
    Stop(usize,usize),
    D(usize, usize),
    Dl(usize, usize),
    Dr(usize, usize),
}

impl NMove {
    fn get(self) -> (usize, usize) {
        match self {
            NMove::New(y, x) => { (y, x) }
            NMove::Stop(y,x) => { (y, x) }
            NMove::D(y, x) => { (y, x) }
            NMove::Dl(y, x) => { (y, x) }
            NMove::Dr(y, x) => { (y, x) }
        }
    }
}

struct Map {
    x: usize,
    y: usize,
    gen: Vec<Vec<Cmd>>,
    map: Vec<Vec<MapObj>>,
    curr: Option<(usize, usize)>,
}

impl Map {
    fn new(cmd_list: &Vec<Vec<Cmd>>) -> Map {
        let mut x = 0;
        let mut y = 0;
        for v in cmd_list {
            for c in v {
                let (newy, newx) = c.get_yx();
                if newy > y {
                    y = newy
                }
                if newx > x {
                    x = newx
                }
            }
        }

        let mut map: Vec<Vec<MapObj>> = vec![];
        for i in 0..y + 1 {
            map.push(vec![]);
            for j in 0..x + 1 {
                map[i].push(MapObj::Air)
            }
        }

        for list in cmd_list {
            let mut iter = list.iter();
            let mut start = iter.next().unwrap();
            let mut end = iter.next().unwrap();
            loop {
                let (yd, xd) = ((end.y - start.y) as i32, (end.x - start.x) as i32);
                if yd != 0 {
                    if yd > 0 {
                        for i in 0..yd {
                            map[(start.y + i) as usize][start.x as usize] = MapObj::Rock;
                        }
                    } else {
                        map[start.y as usize][(start.x) as usize] = MapObj::Rock;
                        for i in yd..0 {
                            map[(start.y + i) as usize][start.x as usize] = MapObj::Rock;
                        }
                    }
                } else if xd != 0 {
                    if xd > 0 {
                        for i in 0..xd {
                            map[start.y as usize][(start.x + i) as usize] = MapObj::Rock;
                        }
                    } else {
                        map[start.y as usize][(start.x) as usize] = MapObj::Rock;
                        for i in xd..0 {
                            map[start.y as usize][(start.x + i) as usize] = MapObj::Rock;
                        }
                    }
                }
                start = end;

                end = match iter.next() {
                    None => { break; }
                    Some(x) => { x }
                }
            }
        }
        map[0][500] = MapObj::Source;

        Map {
            y,
            x,
            map,
            gen: cmd_list.to_vec(),
            curr: None,
        }
    }

    fn get_down(&self, cur: NMove) -> NMove {
        let (y, x) = cur.get();
        return if self.map[y + 1][x] == MapObj::Air {
            NMove::D(y + 1, x)
        } else if self.map[y + 1][x] == MapObj::Sand || self.map[y + 1][x] == MapObj::Rock {
            return if self.map[y + 1][x - 1] == MapObj::Air {
                NMove::D(y + 1, x - 1)
            } else if self.map[y + 1][x + 1] == MapObj::Air {
                NMove::D(y + 1, x + 1)
            } else {
                Stop(y, x)
            }
        } else { Stop(y, x) };
    }

    fn add_sand(&mut self) {
        let mut curr = NMove::New(0, 500);
        loop {
            let lres = self.get_down(curr);
            match lres {
                NMove::New(_, _) => {}
                NMove::D(y, x) => { curr = NMove::New(y, x) }
                NMove::Dl(_, _) => {}
                NMove::Dr(_, _) => {}
                NMove::Stop(y,x) => {
                    self.map[y][x] = MapObj::Sand;
                    break;
                }
            }
        }
    }

    fn print(&self) {
        println!();
        let mut prt_str = String::new();
        for i in 0..self.y + 1 {
            for j in 400..self.x + 1 {
                match self.map[i][j] {
                    MapObj::Air => prt_str.push('.'),
                    MapObj::Rock => prt_str.push('#'),
                    MapObj::Sand => prt_str.push('o'),
                    MapObj::Source => prt_str.push('+')
                }
            }
            prt_str.push('\n')
        }
        println!("{}", prt_str)
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day14demo.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut draw_cmds: Vec<Vec<Cmd>> = vec![];
    let re = Regex::new(r"([0-9]+),([0-9]+)").unwrap();
    for line in input.lines() {
        let mut cmds: Vec<Cmd> = vec![];
        for cap in re.captures_iter(line) {
            println!("y: {} x: {}", &cap[1], &cap[2]);
            cmds.push(Cmd { x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap() })
        }
        draw_cmds.push(cmds)
    }
    print!("{:?}", draw_cmds);
    let mut map = Map::new(&draw_cmds);
    map.print();
    loop {
        map.add_sand();
        map.print();
    }
}