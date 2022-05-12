extern crate board;

use std::collections::{VecDeque, HashSet};

use board::board::Board;


/// #generate
/// 
/// 'generate' is a function that generate 
/// boards of some maximum depth to a given path 
pub fn generate(level: usize, starting_board: Board, path: &str){
    let mut q: VecDeque<Board> = VecDeque::new();
    let mut u: HashSet<Board> = HashSet::new();
    let mut all: Vec<Board> = Vec::new();

    q.push_back(starting_board.clone());
    u.insert(starting_board.clone());

    while !q.is_empty() {
        let v = q.remove(0).unwrap();
        for n in v.find_neighbors( [0, 1, 2, 3]){
            
            if !u.contains(&n) && n.path.len() <= level{
                all.push(n.clone());
                q.push_back(n.clone());
                u.insert(n);
            }
        }

    }

    for (i, b) in all.iter().enumerate(){
        if b.path.len() == 0{
            continue;
        }
        let mut filename = String::new();
        let id: &str = &(&i+1).to_string();
        let id = format!("{}{}", &"0000"[..(4-id.to_string().len())], id);
        filename.push_str(&format!("{}/{}x{}_0{}_{}.txt",path, &b.dimentions.0, &b.dimentions.1,&b.path.len(), id));
        match b.to_file(&filename){
            Ok(()) => println!("Board \"{}\" was saved.", filename),
            Err(error) => println!("An error occur while saving board \"{}\": {}", filename, error)
        }

    }

}
