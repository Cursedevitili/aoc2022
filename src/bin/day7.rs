use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;
use regex::{Captures, Regex};

#[derive(Debug)]
struct  File {
    name: String,
    size: u32
}

impl File {
    fn new (name: String, size: u32) -> File {
        File {
            name,
            size
        }
    }
}

#[derive(Debug)]
struct Dir {
    dirs: Vec<Rc<Dir>>,
    upper_dir: Option<Rc<Dir>>,
    current_dir: Option<Rc<Dir>>,
    files: Vec<File>,
    name: String,
    level: u32
}

impl Dir {
    fn new(name: String, level: u32) -> Dir{

        Dir {
            dirs: Vec::new(),
            upper_dir: None,
            current_dir: None,
            files: Vec::new(),
            name: name.to_string(),
            level
        }
    }


    fn add_dir (&mut self, new: Dir) {
        self.dirs.push(Rc::new(new));
    }

    fn add_file (&mut self, new: File) {
        self.files.push(new);
    }

    fn add_upper(&mut self, upper: Dir){
        self.upper_dir = Some(Rc::new(upper))
    }
}

fn get_string_after(line:&str, skip_how_many: u32) -> String {
    let mut char_iter = line.chars();
    for _ in 0..skip_how_many {
        char_iter.next();
    }
    let mut name: String = String::new();
    loop {
        match char_iter.next() {
            None => { break; }
            Some(x) => {
                name.push(x);
            }
        }
    }
    name
}

fn process_found_line(line: String, current_directory: mut Dir) {
    if line.contains("$ ls"){
        println!("see files"); //Do nothing
    } else if line.contains("$ cd ..") {
        println!("go one folder up");
    } else if line.contains("$ cd") {
        let folder = get_string_after(&line, 5);
        println!("goto folder {}", folder);
        for i in 0..current_directory.dirs.len() {
            if current_directory.dirs[i].name == folder {
                return;
            }
        }
    } else {
        println!("Dir or file");
        let re = Regex::new(r"([0-9]+) ([a-zA-Z0-9]+)").unwrap();
        let caps = re.captures(&line);
        match caps {
            None => {
                let dir_name= get_string_after(&line, 4);
                let mut new = Dir::new(dir_name.clone(), current_directory.level+1);
                new.add_upper(current_directory);
                current_directory.add_dir(new);
            }
            Some(x) => {
                current_directory.add_file(File::new(x[2].to_string(), x[1].parse().unwrap()));
            }
        }
    }
}

fn main() {
    let input = aoc2022::shared::load_input("day7demo.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );
    let mut rootdir = Dir::new("root".to_string(), 0);

    let mut iter = input.lines();
    iter.next();

    loop {
        let curr_line = iter.next();
        match curr_line {
            None => {break;}
            Some(matched_line) => {
                process_found_line(matched_line.to_string(), &mut rootdir);
            }
        }
    }

    println!("end")

    // let mut dirb = Dir::new("b", 1);
    // dira.add_dir(dirb.clone());
    // dirb.add_upper(dira);
}