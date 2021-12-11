use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Point {
    fn from_coordinates(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_points(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }
}

fn parse_line(l: &str) -> Result<Line, String> {
    let mut numbers_str = vec!["".to_string(); 4];
    let mut numbers_index = 0;

    l.chars().into_iter().for_each(|c| match c {
        ',' => numbers_index += 1,
        '>' => numbers_index += 1,
        ' ' => {}
        '-' => {}
        _ => {
            numbers_str[numbers_index].push(c);
        }
    });

    let mut numbers = [0u32; 4];

    for (index, number_str) in numbers_str.iter().enumerate() {
        numbers[index] = match number_str.parse() {
            Ok(n) => n,
            Err(_) => return Err("Failed at parse_line!".to_string()),
        }
    }

    Ok(Line::from_points(
        Point::from_coordinates(numbers[0], numbers[1]),
        Point::from_coordinates(numbers[2], numbers[3]),
    ))
}

fn part1() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut smoke_coordinates: HashMap<Point, usize> = HashMap::new();

    for line_result in reader.lines() {
        let line_str = match line_result {
            Ok(l) => l,
            Err(_) => return,
        };

        let line = match parse_line(&line_str) {
            Ok(l) => l,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        if line.is_horizontal() {
            let x = line.start.x;
            let mut start_y = line.start.y;
            let mut end_y = line.end.y;

            if start_y > end_y {
                let tmp = start_y;
                start_y = end_y;
                end_y = tmp;
            }
            for y in (start_y..end_y + 1).into_iter() {
                let p = Point::from_coordinates(x, y);
                *smoke_coordinates.entry(p).or_insert(0) += 1;
            }
        } else if line.is_vertical() {
            let y = line.start.y;
            let mut start_x = line.start.x;
            let mut end_x = line.end.x;

            if start_x > end_x {
                let tmp = start_x;
                start_x = end_x;
                end_x = tmp;
            }

            for x in (start_x..end_x + 1).into_iter() {
                let p = Point::from_coordinates(x, y);
                *smoke_coordinates.entry(p).or_insert(0) += 1;
            }
        }
    }

    let mut num_overlaps = 0u32;

    for (_, number) in smoke_coordinates {
        if number > 1 {
            num_overlaps += 1;
        }
    }

    println!("\tPart 1: {}", num_overlaps);
}
fn part2() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut smoke_coordinates: HashMap<Point, usize> = HashMap::new();

    for line_result in reader.lines() {
        let line_str = match line_result {
            Ok(l) => l,
            Err(_) => return,
        };

        let line = match parse_line(&line_str) {
            Ok(l) => l,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        if line.is_horizontal() {
            let x = line.start.x;
            let mut start_y = line.start.y;
            let mut end_y = line.end.y;

            if start_y > end_y {
                let tmp = start_y;
                start_y = end_y;
                end_y = tmp;
            }
            for y in (start_y..end_y + 1).into_iter() {
                let p = Point::from_coordinates(x, y);
                *smoke_coordinates.entry(p).or_insert(0) += 1;
            }
        } else if line.is_vertical() {
            let y = line.start.y;
            let mut start_x = line.start.x;
            let mut end_x = line.end.x;

            if start_x > end_x {
                let tmp = start_x;
                start_x = end_x;
                end_x = tmp;
            }

            for x in (start_x..end_x + 1).into_iter() {
                let p = Point::from_coordinates(x, y);
                *smoke_coordinates.entry(p).or_insert(0) += 1;
            }
        } else {
            let mut start_x = line.start.x;
            let mut end_x = line.end.x;
            let mut rev_x = false;

            if start_x > end_x {
                let tmp = start_x;
                start_x = end_x;
                end_x = tmp;
                rev_x = true;
            }

            let mut start_y = line.start.y;
            let mut end_y = line.end.y;
            let mut rev_y = false;

            if start_y > end_y {
                let tmp = start_y;
                start_y = end_y;
                end_y = tmp;
                rev_y = true;
            }

            for (y, x) in (start_y..end_y + 1)
                .into_iter()
                .zip((start_x..end_x + 1).into_iter())
            {
                // let p = Point::from_coordinates(x, y);
                let p = match (rev_x, rev_y) {
                    (true, false) => Point::from_coordinates(end_x + start_x - x, y),
                    (true, true) => Point::from_coordinates(end_x + start_x  - x, end_y + start_y - y),
                    (false, true) => Point::from_coordinates(x, end_y + start_y - y ),
                    (false, false) => Point::from_coordinates(x, y),
                };

                *smoke_coordinates.entry(p).or_insert(0) += 1;
            }
        }
    }

    let mut num_overlaps = 0u32;

    for (_, number) in smoke_coordinates {
        if number > 1 {
            num_overlaps += 1;
        }
    }

    println!("\tPart 2: {}", num_overlaps);
}

fn main() {
    println!("Problem 5:");
    part1();
    part2();
}
