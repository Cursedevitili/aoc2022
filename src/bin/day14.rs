use regex::Regex;

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
    Route,
    Source,
}

#[derive(Clone, Copy)]
enum NMove {
    New(usize, usize),
    Stop(usize, usize),
    Abyss,
    SourceFilled,
    D(usize, usize),
    Dl(usize, usize),
    Dr(usize, usize),
}

impl NMove {
    fn get(self) -> (usize, usize) {
        match self {
            NMove::New(y, x) => { (y, x) }
            NMove::Stop(y, x) => { (y, x) }
            NMove::D(y, x) => { (y, x) }
            NMove::Dl(y, x) => { (y, x) }
            NMove::Dr(y, x) => { (y, x) }
            _ => { (0, 0) }
        }
    }
}

struct Map {
    x: usize,
    y: usize,
    draw_lim_y: usize,
    draw_lim_x: usize,
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
        for i in 0..y + 1 + 2 + 1 { // (y + 1 + 2 + 1) Highest Y + 2 for floor and 1 to fot the floor
            map.push(vec![]);
            for j in 0..x + x + 1 { // (x + x + 1)Double map width
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
                        for i in 0..yd + 1 {
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
                        for i in 0..xd + 1 {
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
            draw_lim_x: 450,
            draw_lim_y: 0,
            map,
            gen: cmd_list.to_vec(),
            curr: None,
        }
    }

    fn get_down(&mut self, cur: NMove) -> NMove {
        let (y, x) = cur.get();
        if y == self.y && self.map[self.y+2][x] != MapObj::Rock{
            return NMove::Abyss;
        } else if self.map[0][500] == MapObj::Sand {
            return NMove::SourceFilled;
        }

        return if self.map[y + 1][x] == MapObj::Air || self.map[y + 1][x] == MapObj::Route {
            self.map[y + 1][x] = MapObj::Route;
            NMove::D(y + 1, x)
        } else if self.map[y + 1][x] == MapObj::Sand || self.map[y + 1][x] == MapObj::Rock {
            return if self.map[y + 1][x - 1] == MapObj::Air || self.map[y + 1][x - 1] == MapObj::Route {
                self.map[y + 1][x - 1] = MapObj::Route;
                NMove::D(y + 1, x - 1)
            } else if self.map[y + 1][x + 1] == MapObj::Air || self.map[y + 1][x + 1] == MapObj::Route {
                self.map[y + 1][x + 1] = MapObj::Route;
                NMove::D(y + 1, x + 1)
            } else {
                NMove::Stop(y, x)
            };
        } else { NMove::Stop(y, x) };
    }

    fn add_sand(&mut self) -> bool {
        let mut curr = NMove::New(0, 500);
        loop {
            let lres = self.get_down(curr);
            match lres {
                NMove::Abyss => { return false; }
                NMove::New(_, _) => {}
                NMove::D(y, x) => { curr = NMove::New(y, x) }
                NMove::Dl(_, _) => {}
                NMove::Dr(_, _) => {}
                NMove::Stop(y, x) => {
                    self.map[y][x] = MapObj::Sand;
                    return true;
                }
                NMove::SourceFilled => { return false; }
            }
        }
    }

    fn add_x_numbers(&self, row: usize, s: &mut String) {
        let v = row / 100;
        let v: char = v.to_string().chars().next().unwrap();
        s.push(v);
        let k = (row % 100) / 10;
        let k: char = k.to_string().chars().next().unwrap();
        s.push(k);
        let y = row % 10;
        let y: char = y.to_string().chars().next().unwrap();
        s.push(y);
    }

    fn add_Y_numbers(&self, s: &mut String) {
        s.push(' ');
        s.push(' ');
        s.push(' ');
        for i in 0..self.x + self.x + 1 {
            let h = i / 100;
            let h: char = h.to_string().chars().next().unwrap();
            s.push(h)
        }
        s.push('\n');
        s.push(' ');
        s.push(' ');
        s.push(' ');
        for i in 0..self.x + self.x + 1 {
            let h = (i % 100) / 10;
            let h: char = h.to_string().chars().next().unwrap();
            s.push(h)
        }
        s.push('\n');
        s.push(' ');
        s.push(' ');
        s.push(' ');
        for i in 0..self.x + self.x + 1 {
            let h = i % 10;
            let h: char = h.to_string().chars().next().unwrap();
            s.push(h)
        }
        s.push('\n');
    }

    fn print(&self) {
        println!();
        let mut prt_str = String::new();
        self.add_Y_numbers(&mut prt_str);

        for i in self.draw_lim_y..self.y + 2 + 1 {
            self.add_x_numbers(i, &mut prt_str);
            for j in 0..self.x + self.x + 1 {
                match self.map[i][j] {
                    MapObj::Air => prt_str.push('.'),
                    MapObj::Rock => prt_str.push('#'),
                    MapObj::Sand => prt_str.push('o'),
                    MapObj::Source => prt_str.push('+'),
                    MapObj::Route => prt_str.push('~')
                }
            }
            prt_str.push('\n')
        }
        println!("{}", prt_str)
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day14.txt").unwrap_or_else(
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
    let mut count = 0;
    loop {
        let res = map.add_sand();
        // clearscreen::clear().unwrap();
        if res == false {
            break;
        }
        // map.print();
        // let mut line = String::new();
        // println!("next");
        // std::io::stdin().read_line(&mut line);
        count = count + 1;
    }
    map.print();
    println!("Count for fall to abyss: {}", count);
    for i in 0 .. map.x + map.x + 1{
        map.map[map.y+2][i] = MapObj::Rock;
    }

    map.print();
    loop {
        let res = map.add_sand();
        if res == false {
            break;
        }
        count = count + 1;
    }
    map.print();
    println!("Count for source to fill: {}", count);
}