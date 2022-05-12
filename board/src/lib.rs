//! # board
//!
//! 'board' is a crate containing various funcionalities
//! of a `fifteen` board, like: finding posible moves,
//! moving tiles and validating board.

pub mod board {
    use std::{
        fmt,
        fs::File,
        io::{Read, Write}, hash::{Hash, Hasher}, cmp::Ordering,
    };

    ///# Board
    ///
    /// 'Boards' is a struct that represent board for popular game 'fifteen'
    #[derive(Debug)]
    pub struct Board {
        pub dimentions: (u8, u8),
        pub tiles: Vec<u8>,
        pub path: String,
        pub score: usize,
    }

    /// basic utilities
    impl Board {
        /// Create a new instance of Board
        ///
        /// # Examples
        ///
        ///
        /// let new_board = Board::new();
        ///
        ///
        pub fn new4x4() -> Self {
            Board {
                dimentions: (4, 4),
                tiles: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0],
                path: String::new(),
                score: 0
            }
        }
        pub fn new(dimentions: (u8, u8)) -> Self {
            let mut t: Vec<u8> = Vec::new();
            let number_of_tiles = (dimentions.0 * dimentions.1)-1;
            for i in 0..number_of_tiles{
                t.push(i+1);
            }
            t.push(0);
            Board {
                dimentions,
                tiles: t,
                path: String::new(),
                score: 0
            }
        }
        /// Create a new instance of Board from vector of u8
        ///
        /// # Examples
        ///
        /// ```
        /// let t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        /// let new_board = board::board::Board::new_from((4, 4), t.clone()).unwrap();
        ///
        /// assert_eq!(&new_board.tiles, &t);
        ///
        /// ```

        pub fn new_from(dimentions: (u8, u8), tiles: Vec<u8>) -> Option<Self> {
            if tiles.len() == (dimentions.0 * dimentions.1).into() {
                return Some(Board {
                    dimentions,
                    tiles,
                    path: String::new(),
                    score: 0
                });
            }
            None
        }

        /// create Board instance from data in file
        /// 
        /// # Examples
        /// 
        /// Board::new_from_file("data.txt")
        pub fn new_from_file(filename: &str) -> std::io::Result<Option<Self>> {
            let mut file: File = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let mut lines: Vec<&str> = contents.split('\n').collect();
            let dim: Vec<&str> = lines[0].split(' ').collect();
            lines.remove(0);
            let mut tiles: Vec<u8> = Vec::new();
            for l in &lines {
                let x: Vec<&str> = l.split(' ').collect();
                for n in x {
                    match n.parse::<u8>() {
                        Ok(number) => tiles.push(number),
                        Err(_) => {}
                    
                }
                }
            }

            Ok(Some(Board {
                dimentions: (dim[0].parse::<u8>().unwrap(), dim[1].parse::<u8>().unwrap()),
                tiles,
                path: String::new(),
                score: 0
            }))
        }
        /// create a clone of Board
        ///
        /// # Examples
        ///
        /// ```
        ///
        /// let board1 = board::board::Board::new();
        ///
        /// let board2 = board1.clone();
        ///
        /// assert_eq!(board1, board2);
        /// ```
        pub fn clone(&self) -> Self {
            Board {
                dimentions: self.dimentions,
                tiles: self.tiles.clone(),
                path: self.path.clone(),
                score: self.score,
            }
        }

        pub fn add_score(&self, new: usize) -> Self {
            Board {
                dimentions: self.dimentions,
                tiles: self.tiles.clone(),
                path: self.path.clone(),
                score: new
            }
        }
        

        /// return a string representation of tiles on the board in base 16
        ///
        ///  # Examples
        /// ```
        /// let board = board::board::Board::new();
        ///
        /// let s: String = board.to_string();
        ///
        /// assert_eq!(s, String::from("123456789abcdef0"));
        /// ```
        pub fn to_string(&self) -> String {
            let mut x: String = String::new();
            for t in &self.tiles {
                x.push(char::from_digit(*t as u32, 16).unwrap());
            }
            x
        }

        /// write a simple board representation to a specific file
        ///
        /// # Examples
        ///
        /// let board = board::board::Board::new_from((4, 4), t.clone()).unwrap();
        /// board..to_file("filename.txt");
        ///  
        pub fn to_file(&self, filename: &str) -> std::io::Result<()> {
            let mut buffer = File::create(filename)?;
            let mut s = String::new();
            s.push_str(&format!("{} {}", self.dimentions.0, self.dimentions.1));

            for (i, n) in self.tiles.iter().enumerate() {
                if i % (self.dimentions.1 as usize) == 0 {
                    s.push('\n');
                }
                s.push_str(&n.to_string());
                s.push(' ');
            }
            s.push('\n');
            buffer.write_all(s.as_bytes())?;

            Ok(())
        }

        /// write a solution to a given board with its lenght
        ///
        /// # Examples
        ///
        /// let board = board::board::Board::new_from((4, 4), t.clone()).unwrap();
        /// board.result_to_file("filename.txt");
        ///  
        pub fn result_to_file(&self, filename: &str) -> std::io::Result<()> {
            let mut buffer = File::create(filename)?;
            let mut s = String::new();

            if self.path == "" {
                s.push_str(&format!("{}", self.path.len()));
            } else {
                s.push_str(&format!("{}", self.path.len()));
                s.push_str(&format!("{}", self.path));
            }

            buffer.write_all(s.as_bytes())?;

            Ok(())
        }
    }

    /// moving
    impl Board {
        /// return a position of empty tile on the board
        ///
        /// # Exapmples
        ///
        /// ```
        /// let board = board::board::Board::new();
        ///
        /// let zero = board.find_zero();
        ///
        /// assert_eq!(zero, 15);
        /// ```
        pub fn find_zero(&self) -> Option<usize> {
            self.tiles.iter().position(|&x| x == 0)
        }

        /// returns a clone of board where empty space was moved to the left
        ///
        /// # Examples
        ///
        /// ```
        /// let board = board::board::Board::new();
        ///
        /// let board_moved = board.left(15).unwrap();
        ///
        /// assert_eq!(&board_moved.tiles, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15]);
        /// assert_eq!(&board_moved.path, &"L");
        /// ```
        ///
        /// ```
        /// let t = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        /// let board = board::board::Board::new_from((4, 4), t.clone()).unwrap();
        ///
        /// let board_moved = board.left(0);
        ///
        /// assert_eq!(board_moved, None);
        /// ```
        pub fn left(&self, zero: usize) -> Option<Board> {
            if zero % self.dimentions.1 as usize == 0 {
                return None;
            }
            let mut b1 = self.clone();
            b1.tiles[zero] = b1.tiles[zero - 1];
            b1.tiles[zero - 1] = 0;
            b1.path.push('L');
            Some(b1)
        }

        /// returns a clone of board where empty space was moved up
        ///
        /// # Examples
        ///
        /// ```
        /// let board = board::board::Board::new();
        ///
        /// let board_moved = board.up(15).unwrap();
        ///
        /// assert_eq!(&board_moved.tiles, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 13, 14, 15, 12]);
        /// assert_eq!(&board_moved.path, &"U");
        /// ```
        ///
        /// ```
        /// let t = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        /// let board = board::board::Board::new_from((4, 4), t.clone()).unwrap();
        ///
        /// let board_moved = board.up(0);
        ///
        /// assert_eq!(board_moved, None);
        /// ```
        pub fn up(&self, zero: usize) -> Option<Board> {
            if zero < self.dimentions.1 as usize {
                return None;
            }
            let mut b1 = self.clone();
            b1.tiles[zero] = b1.tiles[zero - 4];
            b1.tiles[zero - 4] = 0;
            b1.path.push('U');
            Some(b1)
        }

        /// returns a clone of board where empty space was moved to the right
        ///
        /// # Examples
        ///
        /// ```
        /// let t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15];
        /// let board = board::board::Board::new_from((4, 4), t.clone()).unwrap();
        ///
        /// let board_moved = board.right(14).unwrap();
        ///
        /// assert_eq!(&board_moved.tiles, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]);
        /// assert_eq!(&board_moved.path, &"R");
        /// ```
        /// 
        /// ```
        /// let board = board::board::Board::new();
        ///
        /// let board_moved = board.right(15);
        ///
        /// assert_eq!(board_moved, None);
        /// ```

        pub fn right(&self, zero: usize) -> Option<Board> {
            if zero % self.dimentions.1 as usize == (self.dimentions.1 - 1) as usize {
                return None;
            }
            let mut b1 = self.clone();
            b1.tiles[zero] = b1.tiles[zero + 1];
            b1.tiles[zero + 1] = 0;
            // println!("R");
            b1.path.push('R');
            Some(b1)
        }

        /// returns a clone of board where empty space was moved down
        ///
        /// # Examples
        ///
        /// ```
        /// let t = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        /// let board = board::board::Board::new_from((4, 4), t.clone()).unwrap();
        ///
        /// let board_moved = board.down(0).unwrap();
        ///
        /// assert_eq!(&board_moved.tiles, &vec![4, 1, 2, 3, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        /// assert_eq!(&board_moved.path, &"D");
        /// ```
        ///
        /// ```
        /// let board = board::board::Board::new();
        ///
        /// let board_moved = board.down(15);
        ///
        /// assert_eq!(board_moved, None);
        /// ```
        pub fn down(&self, zero: usize) -> Option<Board> {
            if zero >= ((self.dimentions.0 * self.dimentions.1) - self.dimentions.1).into() {
                return None;
            }
            let mut b1 = self.clone();
            b1.tiles[zero] = b1.tiles[zero + 4];
            b1.tiles[zero + 4] = 0;
            // println!("D");
            b1.path.push('D');
            Some(b1)
        }

        /// function that finds all posible moves for board and return moved boards
        /// order of returned neighbors is according to elements in passed array argument
        /// 0: right
        /// 1: up
        /// 2: left
        /// 3: down
        ///
        /// # Examples
        ///
        /// ```
        /// let board = board::board::Board::new();
        /// let neighbors = board.find_neighbors([0, 1, 2, 3]);
        ///
        /// assert_eq!(neighbors.len(), 2);
        /// assert_eq!(&neighbors[0].tiles, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15]);
        /// assert_eq!(&neighbors[1].tiles, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 13, 14, 15, 12]);
        /// ```
        pub fn find_neighbors(&self, directions: [usize; 4]) -> Vec<Board> {
            let functions = [Board::left, Board::up, Board::right, Board::down];
            let mut n: Vec<Board> = Vec::new();
            let zero = self.find_zero();
            for i in directions {
                let x = functions[i](&self, zero.unwrap());
                if !x.is_none() {
                    n.push(x.unwrap());
                }
            }
            n
        }
    }

    impl PartialEq for Board {
        fn eq(&self, other: &Self) -> bool {
            self.tiles == other.tiles
        }
    }

    impl Eq for Board {}

    impl Ord for Board {
        fn cmp(&self, other: &Self) -> Ordering {

            self.score.cmp(&other.score)
        }
    }

    impl PartialOrd for Board {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }


    impl Hash for Board {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.tiles.hash(state);
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut x: String = String::new();
            let mut i: u8 = 0;
            x.push('\n');
            for t in &self.tiles {
                x.push(char::from_digit(*t as u32, 16).unwrap());
                x.push('|');
                i += 1;
                if i % 4 == 0 {
                    x.push('\n');
                }
            }

            write!(f, "{}", x)
        }
    }
}
