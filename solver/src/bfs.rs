use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use board::board::Board;

use crate::{write_result, write_stats};

pub fn solve(directions: [usize; 4], source: String, solution: String, stats: String) {

    let board: Board;

    match Board::new_from_file(&source) {
        Err(e) => {
            println!("cannot open source file: {}", e);
            return;
        }
        Ok(o_board) => match o_board {
            None => {
                println!("invalid source file");
                return;
            }
            Some(b) => {
                board = b;
            }
        },
    }

    let time = Instant::now();
    let res = bfs_alg(Board::new(board.dimentions), &board, directions);
    let time = time.elapsed().as_micros();
    match write_result(&solution, &res.0) {
        Err(e) => {
            println!("Cannot write solution to the file {}: {}", solution, e);
            return;
        }
        Ok(()) => {}
    }

    match write_stats(
        &stats,
        res.0,
        res.1,
        res.2,
        res.3,
        time,
    ) {
        Err(e) => {
            println!("Cannot write solution to the file {}: {}", solution, e);
            return;
        }
        Ok(()) => {}
    }
}

fn bfs_alg(g: Board, s: &Board, d: [usize; 4]) -> (String, usize, usize, usize) {
    if g == *s {
        return (s.path.clone(), 1, 1, 0);
    }
    let mut processed = 1;

    let mut q: VecDeque<Board> = VecDeque::new();
    let mut u: HashSet<Board> = HashSet::new();
    q.push_back(s.clone());
    u.insert(s.clone());
    let mut md = s.path.len();

    while !q.is_empty() {
        let v = q.remove(0).unwrap();
        processed += 1;
        for n in v.find_neighbors(d) {
            if n.path.len() > md{
                md = n.path.len();
            }
            if g == n {
                return ((n.path).to_string(), u.len(), processed, md);
            }
            if !u.contains(&n) {
                q.push_back(n.clone());
                u.insert(n);
            }
        }
    }
    let mut md = 0;

    for x in &u {
        if x.path.len() > md {
            md = x.path.len();
        }
    }

    return ("-1".to_string(), u.len(), processed, md);
}