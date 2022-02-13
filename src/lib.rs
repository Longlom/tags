#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Instant};

////////////////////////////////////////////////////////////////////////////////

/// Represents a tile on a board. A tile can either be empty or a number from 1 to 8.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tile(u8);

impl Tile {
    /// Creates a new tile.
    ///
    /// # Arguments
    ///
    /// * `maybe_value` - Some(1..=8) or None.
    ///
    /// # Panics
    ///
    /// Panics if value is 0 or > 8.
    pub fn new(maybe_value: Option<u8>) -> Self {
        let value = maybe_value.unwrap();
        assert_eq!(value > 0 && value < 9, true);
        return Self(value);
    }

    /// Creates an empty tile.
    pub fn empty() -> Self {
        return Self(0);
    }

    /// Returns `Some(value)` if tile contains a value, otherwise returns `None`.
    pub fn number(&self) -> Option<u8> {
        if self.0 == 0 {
            return None;
        }
        return Some(self.0);
    }

    /// Returns true if tile does not contain a value.
    pub fn is_empty(&self) -> bool {
        // TODO: your code here.
        if self.0 == 0 {
            return true;
        }
        return false;
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Represents a 3x3 board of tiles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    tiles: [[Tile; 3]; 3],
}

impl Board {
    /// Creates a new `Board` from a 3x3 matrix if `Tile`s.
    ///
    /// # Panics
    ///
    /// Panics if `tiles` contains more than one instance if some tile.
    pub fn new(tiles: [[Tile; 3]; 3]) -> Self {
        // TODO: your code here.
        let set = tiles.iter().flatten().cloned().collect::<HashSet<_>>();
        assert_eq!(set.len(), 9);
        return Board { tiles };
    }

    /// Returns a tile on a given `row` and `col`.
    ///
    /// # Panics
    ///
    /// Panics if `row` or `col` > 2.
    pub fn get(&self, row: usize, col: usize) -> Tile {
        // TODO: your code here.
        assert_eq!(row < 3 && col < 3, true);
        return self.tiles[row][col];
    }

    /// Swaps two given tiles.
    ///
    /// # Panics
    ///
    /// Panics if some of `r1`, `r2`, `c1` or `c2` > 2.
    pub fn swap(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
        assert_eq!(r1 < 3 && c1 < 3 || r2 < 3 && c2 < 3, true);
        let temp_tile: Tile = self.tiles[r1][c1];
        self.tiles[r1][c1] = self.tiles[r2][c2];
        self.tiles[r2][c2] = temp_tile;
    }

    /// Parses `Board` from string.
    ///
    /// # Arguments
    ///
    /// * `s` must be a string in the following format:
    ///
    /// '''
    /// .12
    /// 345
    /// 678
    /// '''
    ///
    /// # Panics
    ///
    /// Panics of `s` is the wrong format or does not represent a valid `Board`.
    pub fn from_string(s: &str) -> Self {
        let mut tiles = [[Tile::empty(); 3]; 3];
        for (i, line) in s.split('\n').take(3).enumerate() {
            for (j, chr) in line.chars().take(3).enumerate() {
                if (chr as u8) == 46 || (chr as u8) > 48 && (chr as u8) < 57 {
                    if chr == '.' {
                        tiles[i][j] = Tile::empty();
                    } else {
                        tiles[i][j] = Tile::new(Some(chr as u8 - 48))
                    }
                } else {
                    panic!("Strange char")
                }
            }
        }
        let board = Board::new(tiles);
        return board;
    }

    /// Returns a string representation of this board in the following format:
    ///
    /// '''
    /// .12
    /// 345
    /// 678
    /// '''
    pub fn to_string(&self) -> String {
        let tiles: Vec<Tile> = self.tiles.iter().flatten().cloned().collect();
        let mut string: String = String::from("");
        for i in 0..tiles.len() {
            match tiles[i].0 {
                1 => string.push('1'),
                2 => string.push('2'),
                3 => string.push('3'),
                4 => string.push('4'),
                5 => string.push('5'),
                6 => string.push('6'),
                7 => string.push('7'),
                8 => string.push('8'),
                0 => string.push('.'),
                _ => {}
            }
            if i % 3 == 2 && i != 8 {
                string.push('\n');
            }
        }
        return string;
    }

    /// Returning true if board is solved otherwise false
    pub fn is_solved(&self) -> bool {
        let mut is_solved = true;
        let tiles: Vec<Tile> = self.tiles.iter().flatten().cloned().collect();
        for i in 0..tiles.len() {
            if i+1 != tiles[i].0.into() && i != 8 {
                is_solved = false;
            } else if i == 8 && tiles[i].0 != 0 {
                is_solved = false;
            }
        }

        return is_solved;
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Returns the shortest sequence of moves that solves this board.
/// That is, a sequence of boards such that each consecutive board can be obtained from
/// the previous one via a single swap of an empty tile with some adjacent tile,
/// and the final board in the sequence is
///
/// '''
/// 123
/// 456
/// 78.
/// '''
///
/// If the board is unsolvable, returns `None`. If the board is already solved,
/// returns an empty vector.
pub fn solve(start: Board) -> Option<Vec<Board>> {
    let moves: Vec<Vec<usize>> = vec![
        vec![1, 3],
        vec![0, 2, 4],
        vec![1, 5],
        vec![0, 4, 6],
        vec![1, 3, 5, 7],
        vec![2, 4, 8],
        vec![3, 7],
        vec![4, 6, 8],
        vec![5, 7],
    ];
    let mut queue: VecDeque<Board> = VecDeque::new();
    // let mut visited: Vec<Board> = Vec::new();
    let mut hash_map: HashMap<Board, Board> = HashMap::new();
    if start.is_solved() {
        println!("INPUT IS SOLVED");
        return Some(Vec::new());
    } else {
        println!("INPUT IS NOT SOLVED");
        
    }

    queue.push_back(start);
    hash_map.insert(start, start);
    let start = Instant::now();
    while let Some(board) = queue.pop_front() {
        let tiles = board.tiles.iter().flatten().cloned().collect::<Vec<_>>();
        
        for (i, tile) in tiles.iter().enumerate() {
            if tile.0 == 0 {
                for mv in &moves[i] {
                    let r1 = i / 3;
                    let c1 = i % 3;
                    let r2 = *mv / 3;
                    let c2 = *mv % 3;
                    let mut copy_board = board;
                    copy_board.swap(r1, c1, r2, c2); // making new state

                    if !hash_map.contains_key(&copy_board) { // if the state wasn't visited
                        hash_map.insert(copy_board, board); // inserting to hash_map
                        queue.push_back(copy_board); // pushing to queue new state
                        if copy_board.is_solved() { // if we reached to result
                            let mut ans: Vec<Board> = Vec::new(); // vec of path to result
                            let mut current_state: Board = copy_board;
                            ans.push(current_state);
                            loop {
                                match hash_map.get(&current_state) {
                                    Some(prev_state) => {
                                        if *prev_state != current_state {
                                        current_state = *prev_state;
                                        ans.insert(0, current_state);
                                        } else {
                                            break;
                                        }
                                    }
                                    None => break,
                                }
                            }
                            ans.pop();
                            let duration = start.elapsed();
                            println!("DURATION{:?}", duration);
                            return Some(ans);
                        }
                    }
                }
            }
        }
    }
    return None;
}