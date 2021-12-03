use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

fn main() {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let numbers: Vec<i32> =
        buf.lines()
           .map(|l| l.expect("Could not parse line").parse::<i32>().expect("could not convert line to number"))
           .collect();
    let mut count_over = 0;
    let mut prev = 9999;
    for num in numbers {
        if num > prev {
            count_over += 1;
        }
        prev = num;
    }
    println!("Result: {}", count_over)
}
