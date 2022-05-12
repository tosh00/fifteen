
use std::{time::Instant, collections::HashSet};

use board::board::Board;

use crate::{write_result, write_stats};

pub fn solve(directions: [usize; 4], source: String, solution: String, stats: String){
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

    let mut all: HashSet<Board> = HashSet::new();
    let mut processed: usize = 0;
    let mut visited: usize = 1;
    let mut md: usize = 0;

    let time = Instant::now();
    let res = dfs_alg(&Board::new(board.dimentions), &board, directions, &mut all, 20, &mut processed, &mut md, &mut visited);
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

fn dfs_alg(g: &Board, v: &Board, d: [usize; 4], t: &mut HashSet<Board>, level: usize, processed: &mut usize, md: &mut usize, visited: &mut usize) -> (String, usize, usize, usize){
    
    
    if v.path.len() > *md{
        *md = v.path.len();
    }
    if v == g {

        return (v.path.clone(), *visited, *processed,* md)
    }
    t.insert(v.clone());
    *processed += 1;
    for n in v.find_neighbors(d){
        
        if level >= n.path.len() && !t.contains(&n){
            *visited+=1;
            let r = dfs_alg(g, &n, d, t, level, processed, md, visited);
            if r.0 != "-1"{
                return r;
            }
        }
    }
    return ("-1".to_owned(), *visited, *processed,* md);
}