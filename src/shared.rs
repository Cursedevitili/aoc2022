use std::{fs, io};

pub fn adder(val1: &i32, val2: &i32) -> i32 {
    val1 + val2
}

pub fn load_input(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(format!("./../input/{}", file_name))
}

pub fn write_output(file_name: &str, output_to_write: &str) ->Result<(), io::Error>{
    let path = format!("./../output/{}", file_name);
    fs::write(path,output_to_write)?;
    Ok(())
}