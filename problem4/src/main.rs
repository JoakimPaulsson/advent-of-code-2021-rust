use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn col_sums(scores_board: &[u32; 25]) -> [u32; 5] {
    let mut buffer = [0u32; 5];

    scores_board.chunks(5).for_each(|row| {
        row.iter()
            .enumerate()
            .for_each(|(i, elem)| buffer[i] += elem);
    });

    buffer
}

fn has_winning_col(scores_board: &[u32; 25]) -> bool {
    match col_sums(&scores_board).iter().find(|&&item| item == 5) {
        Some(_) => true,
        None => false,
    }
}

fn has_winning_row(scores_board: &[u32; 25]) -> bool {
    let mut tr_board = [0u32; 25];

    for j in 0..5 {
        for i in 0..5 {
            tr_board[j + i * 5] = scores_board[i + j * 5];
        }
    }

    has_winning_col(&tr_board)
}

fn part1() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut lines_it = reader.lines();
    let mut boards: Vec<[u32; 25]> = Vec::default();
    let mut board_buff = [0u32; 25];

    if let Some(first_line_result) = lines_it.next() {
        if let Ok(first_line) = first_line_result {
            lines_it.enumerate().for_each(|(line_nr, line_result)| {
                let line_index = line_nr % 6;

                if let Ok(line) = line_result {
                    match line_index {
                        0 => {
                            // empty line
                        }
                        5 => {
                            for (i, num_str) in line.split_whitespace().enumerate() {
                                if let Ok(num) = num_str.parse::<u32>() {
                                    board_buff[i + (line_index - 1) * 5] = num;
                                }
                            }
                            boards.push(board_buff);
                            board_buff = [0u32; 25];
                        }
                        _ => {
                            for (i, num_str) in line.split_whitespace().enumerate() {
                                if let Ok(num) = num_str.parse::<u32>() {
                                    board_buff[i + (line_index - 1) * 5] = num;
                                }
                            }
                        }
                    }
                }
            });

            let mut scores = vec![[0u32; 25]; boards.len()];

            for num_str in first_line.split(',') {
                if let Ok(num) = num_str.parse::<u32>() {
                    for (ib, &board) in boards.iter().enumerate() {
                        match board.iter().enumerate().find(|(_, cell)| **cell == num) {
                            Some((ic, _)) => scores[ib][ic] = 1,
                            None => (),
                        };

                        if has_winning_row(&scores[ib]) || has_winning_col(&scores[ib]) {
                            let sum: u32 = board
                                .iter()
                                .zip(scores[ib].iter())
                                .map(|(&b_elem, &s_elem)| if s_elem == 0 { b_elem } else { 0 })
                                .sum();
                            println!("\tPart 1: {}", sum * num);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn part2() {
    let reader = BufReader::new(match File::open("input.txt") {
        Ok(it) => it,
        _ => return,
    });

    let mut lines_it = reader.lines();
    let mut boards: Vec<[u32; 25]> = Vec::default();
    let mut board_buff = [0u32; 25];

    if let Some(first_line_result) = lines_it.next() {
        if let Ok(first_line) = first_line_result {
            lines_it.enumerate().for_each(|(line_nr, line_result)| {
                let line_index = line_nr % 6;

                if let Ok(line) = line_result {
                    match line_index {
                        0 => {
                            // empty line
                        }
                        5 => {
                            for (i, num_str) in line.split_whitespace().enumerate() {
                                if let Ok(num) = num_str.parse::<u32>() {
                                    board_buff[i + (line_index - 1) * 5] = num;
                                }
                            }
                            boards.push(board_buff);
                            board_buff = [0u32; 25];
                        }
                        _ => {
                            for (i, num_str) in line.split_whitespace().enumerate() {
                                if let Ok(num) = num_str.parse::<u32>() {
                                    board_buff[i + (line_index - 1) * 5] = num;
                                }
                            }
                        }
                    }
                }
            });

            let mut scores = vec![[0u32; 25]; boards.len()];
            let mut last_winning_board_index = 0usize;
            let mut last_winning_num = 0u32;
            let mut has_won_boards_indicies: Vec<usize> = Vec::new();

            for num_str in first_line.split(',') {
                if let Ok(num) = num_str.parse::<u32>() {
                    for (ib, &board) in boards.iter().enumerate() {

                        if has_won_boards_indicies.contains(&ib){
                            continue;
                        }

                        match board.iter().enumerate().find(|(_, cell)| **cell == num) {
                            Some((ic, _)) => scores[ib][ic] = 1,
                            None => (),
                        };

                        if has_winning_row(&scores[ib]) || has_winning_col(&scores[ib]) {
                            has_won_boards_indicies.push(ib);
                            last_winning_board_index = ib;
                            last_winning_num = num
                        }
                    }
                }
            }

            let sum: u32 = boards[last_winning_board_index]
                .iter()
                .zip(scores[last_winning_board_index].iter())
                .map(|(&b_elem, &s_elem)| if s_elem == 0 { b_elem } else { 0 })
                .sum();

            println!("\tPart 2: {}", sum * last_winning_num);
        }
    }
}

fn main() {
    println!("Problem 4:");
    part1();
    part2();
}
