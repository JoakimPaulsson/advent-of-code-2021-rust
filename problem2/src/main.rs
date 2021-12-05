use std::{io::{BufReader, BufRead}, fs::File};

fn part1() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut tot_forward: u32 = 0;
    let mut tot_down: u32 = 0;

    reader
        .lines()
        .for_each(|line_result| {
            if let Ok(line) = line_result {
                if let Some((inst, num_str)) = line.split_once(' '){
                    if let Ok(num) = num_str.parse::<u32>(){
                        match inst {
                            "forward" => tot_forward += num,
                            "down" => tot_down += num,
                            "up" => tot_down -= num,
                            _ => {}
                        }
                    }
                }
            }
        });

    println!("\tPart 1: {}", tot_forward * tot_down);

}

fn part2() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut tot_forward: u32 = 0;
    let mut tot_down: u32 = 0;
    let mut aim: u32 = 0;

    reader
        .lines()
        .for_each(|line_result| {
            if let Ok(line) = line_result {
                if let Some((inst, num_str)) = line.split_once(' '){
                    if let Ok(num) = num_str.parse::<u32>(){
                        match inst {
                            "forward" => {
                                tot_forward += num;
                                tot_down += aim * num
                            },
                            "down" => aim += num,
                            "up" => aim -= num,
                            _ => {}
                        }
                    }
                }
            }
        });

    println!("\tPart 2: {}", tot_forward * tot_down);

}

fn main() {
    println!("Problem 2:");
    part1();
    part2();
}
