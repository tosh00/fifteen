use std::{fs::File, io::Read};

use board::board::Board;

pub fn check(board: String, solution: String) {
    let r = Board::new_from_file(&board);
    let b: Board;
    match r {
        Err(e) => panic!("Cannot read board from file: {}", e),

        Ok(ob) => match ob {
            None => panic!("Board is invalid"),

            Some(board) => b = board,
        },
    }
    let r = open_solution(&solution);
    let s: String;
    match r {
        Err(e) => panic!("cannot open solution file due to: {}", e),
        Ok(p) => s = p,
    }
    let g = Board::new(b.dimentions);

    let mut b2 = b.clone();
    for c in s.chars() {
        match c {
            'L' => {
                b2 = b2.left(b2.find_zero().unwrap()).unwrap();
            }
            'U' => {
                b2 = b2.up(b2.find_zero().unwrap()).unwrap();
            }
            'R' => {
                b2 = b2.right(b2.find_zero().unwrap()).unwrap();
            }
            'D' => {
                b2 = b2.down(b2.find_zero().unwrap()).unwrap();
            }
            _ => println!("invalid path"),
        }
    }
    if b2 == g {
        println!("{} \u{2714}", solution);
    } else {
        println!("{} \u{2717}", solution);
    }
}

fn open_solution(filename: &str) -> std::io::Result<String> {
    let mut contents = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split('\n').collect();
    if lines.len() < 2 {
        panic!("File dose not contain a valid solution")
    }
    let sol = lines[1];

    Ok(sol.to_string())
}
