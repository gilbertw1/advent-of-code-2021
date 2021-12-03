use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::VecDeque
};

fn main() {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let numbers: Vec<i32> =
        buf.lines()
           .map(|l| l.expect("Could not parse line").parse::<i32>().expect("could not convert line to number"))
           .collect();
    let mut count_over = 0;
    let mut prev = 999999;
    let mut queue: VecDeque<i32> = VecDeque::with_capacity(3);
    for num in numbers {
        queue.push_front(num);
        if queue.len() > 3 {
            queue.pop_back();
        }
        if queue.len() == 3 {
            let sum: i32 = queue.iter().sum();
            if sum > prev {
                count_over += 1;
            }
            prev = sum;
        }
    }
    println!("Result: {}", count_over)
}
