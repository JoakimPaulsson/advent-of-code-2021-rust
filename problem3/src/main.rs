use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut ones = [0u32; 12];
    let mut zeros = [0u32; 12];

    reader.lines().for_each(|line_result| {
        if let Ok(line) = line_result {
            for (index, bit) in line.chars().enumerate() {
                match bit {
                    '0' => zeros[index] += 1,
                    '1' => ones[index] += 1,
                    _ => {}
                }
            }
        }
    });

    let mut gamma = 0u32;
    let mut epsilon = 0u32;

    ones.into_iter()
        .zip(zeros.into_iter())
        .enumerate()
        .for_each(|(index, (o, z)): (usize, (u32, u32))| {
            if o > z {
                gamma = (1 << ones.len() - 1 - index) | gamma;
            } else {
                epsilon = (1 << zeros.len() - 1 - index) | epsilon;
            }
        });

    println!("\tPart 1: {}", gamma * epsilon);
}

fn part2() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut oxygen_binary_strings: Vec<String> = Vec::new();
    let mut co2_binary_strings: Vec<String> = Vec::new();

    reader.lines().for_each(|line_result| {
        if let Ok(line) = line_result {
            oxygen_binary_strings.push(line.to_string());
            co2_binary_strings.push(line.to_string());
        }
    });

    let mut index = 0usize;

    while oxygen_binary_strings.len() > 1 {
        let mut zeros = 0usize;
        let mut ones = 0usize;

        oxygen_binary_strings.iter().for_each(|binary| {
            if let Some(c) = binary.chars().nth(index) {
                match c {
                    '1' => ones += 1,
                    '0' => zeros += 1,
                    _ => {}
                }
            }
        });

        let most_common_value = if ones >= zeros { '1' } else { '0' };

        oxygen_binary_strings = oxygen_binary_strings
            .into_iter()
            .filter(|binary| match binary.chars().nth(index) {
                Some(c) => c == most_common_value,
                _ => false,
            })
            .collect();

        index += 1;
    }

    index = 0;

    while co2_binary_strings.len() > 1 {
        let mut zeros = 0usize;
        let mut ones = 0usize;

        co2_binary_strings.iter().for_each(|binary| {
            if let Some(c) = binary.chars().nth(index) {
                match c {
                    '1' => ones += 1,
                    '0' => zeros += 1,
                    _ => {}
                }
            }
        });

        let most_common_value = if ones >= zeros { '0' } else { '1' };

        co2_binary_strings = co2_binary_strings
            .into_iter()
            .filter(|binary| match binary.chars().nth(index) {
                Some(c) => c == most_common_value,
                _ => false,
            })
            .collect();

        index += 1;
    }

    let mut oxygen = 0u32;
    let mut co2 = 0u32;

    oxygen_binary_strings[0]
        .chars()
        .enumerate()
        .for_each(|(cindex, c)| match c {
            '1' => oxygen += 1 << 11 - cindex,
            _ => {}
        });

    co2_binary_strings[0]
        .chars()
        .enumerate()
        .for_each(|(cindex, c)| match c {
            '1' => co2 += 1 << 11 - cindex,
            _ => {}
        });

    println!("\tPart 2: {}", oxygen * co2);

}

fn main() {
    println!("Problem 3:");
    part1();
    part2();
}
