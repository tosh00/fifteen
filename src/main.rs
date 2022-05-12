extern crate board;
extern crate generator;

use std::env;

use board::board::Board;


fn main() {

    match env::args().nth(1) {
        None => {
            help(None);
        },
        Some(command) => {
            match &command[..] {
                "generate" => {
                    if env::args().len() != 4{
                        println!("invalid arguments");
                        help(Some("generate".to_string()));
                        return
                    }
                    let d = env::args().nth(2).unwrap().parse::<usize>();
                    if d.is_err(){
                        println!("<depth> must be a number");
                        help(Some("generate".to_string()));
                    }else{
                        // println!("{}, {}, {}", d.unwrap(), Board::new(), &env::args().nth(3).unwrap()[..]);
                        generator::generate(d.unwrap(), Board::new((4, 4)), &env::args().nth(3).unwrap()[..])
                    }
                }
                "bfs" => {
                    if env::args().len() != 6{
                        println!("invalid arguments");
                        help(Some("bfs".to_string()));
                        return
                    }
                    let d = env::args().nth(2).unwrap();

                    if d.len() != 4 || !d.contains("L") || !d.contains("U") || !d.contains("R") || !d.contains("D") {
                        println!("invalid directions");
                        help(Some("bfs".to_string()));  
                        return
                    }

                    let d = order_to_array(&d);

                    solver::bfs::solve(d, env::args().nth(3).unwrap(), env::args().nth(4).unwrap(), env::args().nth(5).unwrap());
                },
                "dfs" => {
                    if env::args().len() != 6{
                        println!("invalid arguments");
                        help(Some("bfs".to_string()));
                        return
                    }
                    let d = env::args().nth(2).unwrap();

                    if d.len() != 4 || !d.contains("L") || !d.contains("U") || !d.contains("R") || !d.contains("D") {
                        println!("invalid directions");
                        help(Some("bfs".to_string()));  
                        return
                    }

                    let d = order_to_array(&d);

                    solver::dfs::solve(d, env::args().nth(3).unwrap(), env::args().nth(4).unwrap(), env::args().nth(5).unwrap());
                },
                "astr" => {
                    if env::args().len() != 6{
                        println!("invalid arguments");
                        help(Some("bfs".to_string()));
                        return
                    }
                    let m = &env::args().nth(2).unwrap()[..];

                    // if m != "manh" && m != "hamm" {
                    //     println!("invalid method");
                    //     help(Some("bfs".to_string()));  
                    //     return
                    // }
                    let mut mn = 0;
                    match m {
                        "hamm" => {
                            mn = 0;
                        }
                        "manh" => {
                            mn = 1;
                        }
                        _ => {}
                    }
                    solver::astar::solve(mn, env::args().nth(3).unwrap(), env::args().nth(4).unwrap(), env::args().nth(5).unwrap() );    
                },
                "check" => {
                    if env::args().len() != 4{
                        println!("invalid arguments");
                        help(Some("check".to_string()));
                        return
                    }
                    let b = env::args().nth(2).unwrap();
                    let s = env::args().nth(3).unwrap();

                    checker::check(b, s);
                  },
                "help" => help(env::args().nth(2)),
                _ => help(None),
            }
        }
    }

}


fn help(command: Option<String>){

    match command {
    None => print!("
    fifteen help [command]
    fifteen generate <depth> [path]
    fifteen [bfs/dfs] [directions] [source] [solution] [stats]
    fifteen astar [hamm/manh]  [source] [solution] [stats]
        "),
    Some(c) =>{
        match &c[..] {
            "help" => print!("
    fifteen help [command] -
        pring help info for given command
        "),

        "generate" => print!("

    fifteen generate <depth> [path] -
        generates boards

        <depth> -
            maximum depth of board
        [path] -
            path to a directory where boards will be saved
        "),
        "bfs" | "dfs" => print!("
    fifteen [bfs/dfs] [directions] [source] [solution] [stats] -
    solves a board

    [bfs/dfs] -
        alghoritm that will be used
        bfs - breath first search
        dfs - depth first search
    [directions] -
        indicates in what directions alghoritm will 
        search first when searching for posible moves
    [source] - 
        path to a file containing starting board
    [solution] -
        path where solution will be saved
    [stats] -
        path where additional stats will be saved
        "),

        "astr" => print!("
    solves a board with A* alghoritm
    [hamm/manh] -
        metric that will be used when solving board
        hamm - Hamming metric
        manh - Manhatan metric
    [source] - 
        path to a file containing starting board
    [solution] -
        path where solution will be saved
    [stats] -
        path where additional stats will be saved
            "),
        
    
        _ =>     print!("
    fifteen help [command]
    fifteen generate <depth> [path]
    fifteen [bfs/dfs] [directions] [source] [solution] [stats]
    fifteen astar [hamm/manh]  [source] [solution] [stats]
        ") 
        }
    }
        
    }
}


fn order_to_array(source: &str) -> [usize; 4]{
    let mut res: Vec<usize> = Vec::new();
    for c in source.chars(){
        match c {
            'L' => res.push(0),
            'U' => res.push(1),
            'R' => res.push(2),
            'D' => res.push(3),
            _ => res.push(0)
        }
    }
    res.try_into().unwrap_or([0, 1, 2, 3])
}