use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let commands: Vec<(String,i32)> =
        buf.lines()
           .map(|l| split_command(l.expect("Could not parse line")))
           .collect();

    let mut distance = 0;
    let mut depth = 0;

    for command in commands {
        match command.0.as_ref() {
            "forward" => distance += command.1,
            "up" => depth -= command.1,
            "down" => depth += command.1,
            _ => () //ignore
        }
    }
    println!("Result: {}", distance * depth)
}

fn split_command(command_string: String) -> (String, i32) {
    let splitted = command_string.split_once(' ').expect("Incorrect command format");
    (splitted.0.to_string(), splitted.1.parse::<i32>().expect("Invalid number in command"))
}
