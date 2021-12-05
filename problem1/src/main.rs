use std::{fs::File, io::{BufReader, BufRead}, usize};


fn part1() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut num_increasing: u32 = 0;
    let mut previous: u32 = 0;

    reader
        .lines()
        .for_each(|line_result: Result<String, std::io::Error>|
                if let Ok(line) = line_result {
                    if let Ok(current) = line.parse::<u32>() {
                        if previous < current {
                            num_increasing += 1;
                        }
                        previous = current;
                    }
                }
        );

    println!("\tPart 1: {}", num_increasing - 1);
}

#[derive(Debug)]
struct RingBuffer {
    data: [u32; 3],
    head: usize,
    sum: u32,
}

impl RingBuffer {
    fn new() -> Self{
        Self {
            data: [0, 0, 0],
            head: 0,
            sum: 0
        }
    }

    fn insert(&mut self, num: u32){
        let tail = (self.head + 1) % 3;
        self.sum = self.sum - self.data[tail] + num;
        self.data[tail] = num;
        self.head = tail;
    }

    fn sum(&self) -> u32 {
        self.sum
    }
}

fn part2() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut ring_buffer = RingBuffer::new();

    let mut num_increasing: u32 = 0;
    let mut previous: u32 = 0;

    reader
        .lines()
        .for_each(|line_result: Result<String, std::io::Error>|
                  if let Ok(line) = line_result {
                      if let Ok(inp) = line.parse::<u32>() {
                          ring_buffer.insert(inp);
                          if previous < ring_buffer.sum(){
                              num_increasing += 1;
                          }
                          previous = ring_buffer.sum();
                      }
                  }
        );

    println!("\tPart 2: {}", num_increasing - 3);
}

fn main() {
    println!("Problem 1:");
    part1();
    part2();
}
