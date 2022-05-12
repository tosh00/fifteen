use std::{collections::HashSet, time::Instant};
use std::cmp::{Reverse};

use keyed_priority_queue::{KeyedPriorityQueue, Entry};

use board::board::Board;

use crate::{write_result, write_stats};


pub fn solve(method: usize, source: String, solution: String, stats: String) {



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


    let method_list = [hamm, manh];

    if method as usize >= method_list.len(){
        panic!("Wrong method")
    }

    let time = Instant::now();
    let res = astar(Board::new(board.dimentions), &board, method_list[method]);
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


fn astar(g: Board, s: &Board, m: fn(&Board, &Board) -> usize) -> (String, usize, usize, usize){
    let mut processed = 1;

    let mut md:usize = 1;

    if &g==s {
        return (s.path.clone(), 1, 1, 0);
    }
    let mut p: KeyedPriorityQueue<Board , Reverse<usize>> = KeyedPriorityQueue::new();
    let mut t: HashSet<Board> = HashSet::new();
    p.push(s.clone(), Reverse(0));
    while !p.is_empty(){
        // println!("len: {}", p.len());
        let v = p.pop().unwrap().0;

        if v.path.len() > md{
            md = v.path.len();
        }

        processed+=1;
        if g==v{
            return (v.path, t.len(), processed, md);
        }
        t.insert(v.clone());
        let neighbors = v.find_neighbors([0, 1, 2, 3]);
        for n in neighbors{

            if !t.contains(&n){

                let f = m(&n, &g);
                match p.entry(n.clone()){
                    Entry::Vacant(_) =>{
                        p.push(n, Reverse(f));
                    }

                    Entry::Occupied(entry) if *entry.get_priority() < Reverse(f) => {
                        // Have found better path to node in queue
                        entry.set_priority(Reverse(f));
                    }
                    _ => {}
                }
            }
            
        }

    }


    (String::from("-1"), 0, 0, 0)
}

fn hamm(board: &Board, goal_board: &Board) -> usize {
    let mut sum = 0;

    let tiles_number = board.dimentions.0 * board.dimentions.1;
    for i in 0..tiles_number as usize {
        if board.tiles[i] != goal_board.tiles[i] && board.tiles[i] != 0 {
            sum += 1;
        }
    }
    sum + board.path.len()
}

fn manh(board: &Board, goal_board: &Board) -> usize {
    let c = board.dimentions.1;
    let mut sum = 0;

    let tiles_number = (board.dimentions.0 * board.dimentions.1) - 1;
    for i in 0..tiles_number as u8 {
        let pos = board.tiles.iter().position(|&x| x == (i + 1)).unwrap();
        let goal_pos = goal_board.tiles.iter().position(|&x| x == (i + 1)).unwrap();
        let distance = (pos as i16 - goal_pos as i16).abs() as u8;
        sum+= distance%c;
        sum+= distance/c;
    }

    sum as usize + board.path.len()
}

// sum as usize + board.path.len()

