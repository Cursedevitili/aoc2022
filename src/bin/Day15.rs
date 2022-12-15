use regex::Regex;

enum MapObj {
    Nul,
    Forbidden,
    Sensor(i32, i32),
    Beacon(i32, i32),
}

impl MapObj {
    fn get_loc(&self) -> (i32, i32){
        match self {
            MapObj::Nul => { panic!() }
            MapObj::Forbidden => { panic!() }
            MapObj::Sensor(y, x) => {(*y, *x)}
            MapObj::Beacon(y, x) => {(*y, *x)}
        }
    }

    fn get_beacon(&self, yx: (i32, i32)) -> MapObj{
        let (y, x) = self.get_loc();
        MapObj::Beacon(y+yx.0, x+yx.1)
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day14.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut map_objs: Vec<MapObj> = vec![];

    let re = Regex::new(r"Sensor at x=(-*[0-9]+), y=(-*[0-9]+): closest beacon is at x=(-*[0-9]+), y=(-*[0-9]+)").unwrap();
    for line in input.lines(){
        for cap in re.captures_iter(line) {
            let s = MapObj::Sensor(cap[2].parse().unwrap(), cap[1].parse().unwrap());
            let b = s.get_beacon((cap[4].parse().unwrap(), cap[3].parse().unwrap()));
            map_objs.push(s);
            map_objs.push(b);
        }
    }
}