use regex::{Captures, Regex};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Dir {
    dirs: Vec<Box<Dir>>,
    upper_dir: Option<Box<Dir>>,
    files: Vec<File>,
    name: String,
    level: u32
}

impl Dir {
    fn new(name: &str, level: u32) -> Dir {

        Dir {
            dirs: Vec::new(),
            upper_dir: None,
            files: Vec::new(),
            name: name.to_string(),
            level
        }
    }

    fn add_dir (&mut self, new: Dir) {
        self.dirs.push(Box::new(new));
    }

    fn add_file (&mut self, new: File) {
        self.files.push(new);
    }

    fn add_upper(&mut self, upper: Box<Dir>){
        self.upper_dir = Some(upper)
    }
}



fn main() {
    let input = aoc2022::shared::load_input("day7demo.txt").unwrap_or_else(
        |e| {
            println!("{}", e);
            std::process::exit(1)
        }
    );
    let mut rootdir = Box::new(Dir::new("root", 0));

    let mut iter = input.lines();
    iter.next();

    loop {
        let curr_line = iter.next();
        match curr_line {
            None => {break;}
            Some(matched_line) => {
                if matched_line.contains("$ ls"){
                    println!("ls"); //Do nothing
                } else if matched_line.contains("$ cd ..") {
                    println!("goto folder");
                } else if matched_line.contains("$ cd") {
                    println!("see files");
                } else {
                    println!("Dir or file");
                    let re = Regex::new(r"([0-9]+) ([a-zA-Z0-9]+)").unwrap();
                    let caps = re.captures(matched_line);
                    match caps {
                        None => {
                            
                        }
                        Some(x) => {
                            rootdir.add_file(File::new(x[2].to_string(), x[1].parse().unwrap()));
                        }
                    }
                }
            }
        }
    }

    println!("end")

    // let mut dirb = Dir::new("b", 1);
    // dira.add_dir(dirb.clone());
    // dirb.add_upper(dira);
}