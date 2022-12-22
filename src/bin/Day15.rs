use regex::Regex;
use itertools::Itertools;

enum MapObj {
    Nul,
    Forbidden(i32, i32),
    Sensor(i32, i32, i32),
    Beacon(i32, i32),
}

impl MapObj {
    fn get_loc(&self) -> (i32, i32) {
        match self {
            MapObj::Nul => { panic!() }
            MapObj::Forbidden(y, x) => { (*y, *x) }
            MapObj::Sensor(y, x, l) => { (*y, *x) }
            MapObj::Beacon(y, x) => { (*y, *x) }
        }
    }

    fn get_forbidden_coords(&self) -> Vec<(i32, i32)> {
        match self {
            MapObj::Nul => { panic!() }
            MapObj::Forbidden(y, x) => { panic!() }
            MapObj::Beacon(y, x) => { panic!() }
            MapObj::Sensor(y, x, l) => {
                let (y, x) = self.get_loc();
                let perms = (0..l + 1).permutations(2);
                let perms_filtered: Vec<Vec<i32>> = perms
                    .into_iter()
                    .filter(|x| x[0].abs() + x[1].abs() <= *l)
                    .collect();
                let mut y_pos_x_pos: Vec<(i32, i32)> = perms_filtered.iter().map(|x| (x[0], x[1])).collect();
                for a in 1..(l + 1) / 2 {
                    y_pos_x_pos.push((a, a));
                }
                let y_neg_x_pos: Vec<(i32, i32)> = y_pos_x_pos.iter().map(|x| (x.0 * -1, x.1)).collect();
                let y_pos_x_neg: Vec<(i32, i32)> = y_pos_x_pos.iter().map(|x| (x.0, x.1 * -1)).collect();
                let y_neg_x_neg: Vec<(i32, i32)> = y_pos_x_pos.iter().map(|x| (x.0 * -1, x.1 * -1)).collect();
                let mut retvec: Vec<(i32, i32)> = vec![];

                retvec.extend(y_pos_x_pos);
                retvec.extend(y_neg_x_pos);
                retvec.extend(y_pos_x_neg);
                retvec.extend(y_neg_x_neg);
                retvec.iter().map(|z| (z.0 + y, z.1 + x)).collect()
            }
        }
    }
}

fn print_map(min: i32, max: i32, used_map_obj_coords: Vec<(i32, i32, char)>, forbid_coords: Vec<(i32, i32)>) -> String {
    let mut ret_str = String::new();
    ret_str.push(' ');
    ret_str.push(' ');
    for i in min..max + 1 {
        ret_str.push((i % 10).to_string().chars().next().unwrap());
        ret_str.push(' ');
    }
    ret_str.push('\n');
    for i in min..max + 1 {
        ret_str.push((i % 10).to_string().chars().next().unwrap());
        ret_str.push(' ');
        for j in min..max + 1 {
            if used_map_obj_coords.contains(&(i, j, 'B')) {
                ret_str.push('B');
            } else if used_map_obj_coords.contains(&(i, j, 'S')) {
                ret_str.push('S');
            } else if forbid_coords.contains(&(i, j)) {
                ret_str.push('#');
            } else {
                ret_str.push('.');
            }
            ret_str.push(' ');
        }
        ret_str.push('\n');
    }
    ret_str
}

fn main() {
    let input = aoc2022::shared::load_input("day15demo.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );

    let mut used_map_obj_coords: Vec<(i32, i32, char)> = vec![];
    let mut used_forbidden_coords: Vec<(i32, i32)> = vec![];
    let mut map_objs: Vec<MapObj> = vec![];

    let re = Regex::new(r"Sensor at x=(-*[0-9]+), y=(-*[0-9]+): closest beacon is at x=(-*[0-9]+), y=(-*[0-9]+)").unwrap();
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let s_y: i32 = cap[2].parse().unwrap();
            let s_x: i32 = cap[1].parse().unwrap();
            let b_y: i32 = cap[4].parse().unwrap();
            let b_x: i32 = cap[3].parse().unwrap();
            let manhattan_length = (b_y - s_y).abs() + (b_x - s_x).abs();
            let s = MapObj::Sensor(s_y, s_x, manhattan_length);
            let b = MapObj::Beacon(b_y, b_x);
            let (sy, sx) = s.get_loc();
            let (by, bx) = b.get_loc();
            used_map_obj_coords.push((sy, sx, 'S'));
            used_map_obj_coords.push((by, bx, 'B'));
            map_objs.push(s);
            map_objs.push(b);
        }
    }

    // println!("{:?}", used_map_obj_coords);
    let forbid = map_objs[0].get_forbidden_coords();
    for x in forbid {
        used_forbidden_coords.push(x);
    }
    let ret = print_map(-2, 40, used_map_obj_coords, used_forbidden_coords);
    println!("{}", ret);
}