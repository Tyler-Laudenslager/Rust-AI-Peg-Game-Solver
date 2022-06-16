// Author: Tyler Laudenslager
// Project: Final Project
// Professor: Dr. Schwesinger
// Semester: Fall 2021
// Date: 12/15/2021
// Purpose: This program solves any arbitrary
// size peg solitare board producing a move list
// that a person can follow to have only one peg
// left. This game is commonly seen at Cracker Barrel.

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;

// This structure holds two items
// move_list -> The list of moves taken to get
//              to the current board state
// board -> The board object obtained by following
//          moves contained in the move_list.
//definition of a Node structure
#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    move_list: Vec<(usize, usize)>,
    board: Board,
}

//implementation for the Node structure
impl Node {
    //constructor for the Node structure
    fn new(move_list: Vec<(usize, usize)>, board: Board) -> Node {
        Node { move_list, board }
    }
}

// This structure contains all the data needed to
// represent the peg board structure.
// size -> size of the board which is defined by
//         the length of the triangle side.
//
// true_count -> how many pegs we currently have on the board
//               terminal state is defined as true_count being 1
//
// data -> One dimensional array to hold the position of each peg
//         position zero in the array represents position one in the
//         two-dimensional structure of the triangle board.
//
// empty_positions -> represents all the peg positions that have a
//                    value of false.
//
// triangle_numbers -> contains all the numbers that define the end
//                     of each row in the triangle board.
//
// pos_to_coor_board_map -> converts position of the board in one-dimension
//                          to a two-dimensional coordinate. 
//                          13 -> (4,2)
// coor_to_pos_board_map -> converts two-dimensional coordiante back to the
//                          position represented in a one-dimensional array.
//                          (4,2) -> 13
#[derive(Debug, Clone, Eq, PartialEq)]
struct Board {
    size: usize,
    true_count: usize,
    data: Vec<bool>,
    empty_positions: HashSet<usize>,
    triangle_numbers: Vec<usize>,
    pos_to_coor_board_map: HashMap<usize, (usize, usize)>,
    coor_to_pos_board_map: HashMap<(usize, usize), usize>,
}

//n+n-1+n-2...3..2..1
//defines how many items are in a triangle for
//the size of the triangle such that 
//triangle_number(5) -> 15. Where 5 is the size
//of the triangle and 15 is the number of items
//contained in the triangle.
fn triangle_number(size: usize) -> usize {
    return (size * (size + 1)) / 2;
}

impl Board {
    //constructor for new Board object
    fn new(
        size: usize,
        empty_position: usize,
        pos_to_coor_board_map: HashMap<usize, (usize, usize)>,
        coor_to_pos_board_map: HashMap<(usize, usize), usize>,
    ) -> Board {
        let adjust_position = empty_position - 1;
        let mut data = vec![true; triangle_number(size)];
        let triangle_numbers: Vec<usize> = (1..size + 1).
                                          map(|x| (x * (x + 1)) / 2).collect();
        let mut empty_positions = HashSet::new();
        empty_positions.insert(adjust_position + 1);
        let true_count = triangle_number(size) - 1;
        data[adjust_position] = false;
        return Board {
            size,
            true_count,
            data,
            empty_positions,
            triangle_numbers,
            pos_to_coor_board_map,
            coor_to_pos_board_map,
        };
    }
    //print the board out for debugging concerns
    //displays the board in a triangle representation.
    fn _print(&self) {
        for number in 1..triangle_number(self.size) + 1 {
            if self.triangle_numbers.contains(&number) {
                println!("{}", self.data[number - 1]);
            } else {
                print!("{} ", self.data[number - 1]);
            }
        }
        println!("");
    }
    //obtain a list of actions available for the particular board state.
    //[(1,3), (7,9), ... ]
    fn get_actions(&self) -> Vec<(usize, usize)> {
        let mut valid_moves: Vec<(usize, usize)> = Vec::new();
        for empty_position in &self.empty_positions {
            let (row, column) = self.pos_to_coor_board_map[*&empty_position];
            if row >= 2 && column <= row - 2 {
                let row_up = (row - 2, column);
                let row_up_2 = (row - 1, column);
                let column_right = (row, column + 2);
                let column_right_2 = (row, column + 1);
                let value1 = self.data[self.coor_to_pos_board_map[&row_up] - 1];
                let value2 = self.data[self.coor_to_pos_board_map[&row_up_2] - 1];
                if value1 == true && value2 == true {
                    let from = self.coor_to_pos_board_map[&row_up];
                    let to = *empty_position;
                    valid_moves.push((from, to));
                }
                let value1 = self.data[self.coor_to_pos_board_map[&column_right] - 1];
                let value2 = self.data[self.coor_to_pos_board_map[&column_right_2] - 1];
                if value1 == true && value2 == true {
                    let from = self.coor_to_pos_board_map[&column_right];
                    let to = *empty_position;
                    valid_moves.push((from, to));
                }
            }
            if row >= 2 && column >= 2 {
                let column_left = (row, column - 2);
                let column_left_2 = (row, column - 1);
                let row_up_column_left = (row - 2, column - 2);
                let row_up_column_left_2 = (row - 1, column - 1);
                let value1 = self.data[self.coor_to_pos_board_map[&column_left] - 1];
                let value2 = self.data[self.coor_to_pos_board_map[&column_left_2] - 1];
                if value1 == true && value2 == true {
                    let from = self.coor_to_pos_board_map[&column_left];
                    let to = *empty_position;
                    valid_moves.push((from, to));
                }
                let value1 = self.data[self.coor_to_pos_board_map[&row_up_column_left] - 1];
                let value2 = self.data[self.coor_to_pos_board_map[&row_up_column_left_2] - 1];
                if value1 == true && value2 == true {
                    let from = self.coor_to_pos_board_map[&row_up_column_left];
                    let to = *empty_position;
                    valid_moves.push((from, to));
                }
            }
            if row <= self.size - 3 {
                let row_down = (row + 2, column);
                let row_down_2 = (row + 1, column);
                let row_down_column_down = (row + 2, column + 2);
                let row_down_column_down_2 = (row + 1, column + 1);
                let value1 = self.data[self.coor_to_pos_board_map[&row_down] - 1];
                let value2 = self.data[self.coor_to_pos_board_map[&row_down_2] - 1];
                if value1 == true && value2 == true {
                    let from = self.coor_to_pos_board_map[&row_down];
                    let to = *empty_position;
                    valid_moves.push((from, to));
                }
                let value1 = self.data[self.coor_to_pos_board_map[&row_down_column_down] - 1];
                let value2 = self.data[self.coor_to_pos_board_map[&row_down_column_down_2] - 1];
                if value1 == true && value2 == true {
                    let from = self.coor_to_pos_board_map[&row_down_column_down];
                    let to = *empty_position;
                    valid_moves.push((from, to));
                }
            }
        }

        valid_moves
    }
    //perform on action on the board state and return a new board state
    //with the move passed in being applied to the new board state.
    fn action(&mut self, from: usize, to: usize) -> Option<Board> {
        let mut new_board = self.clone();
        new_board.empty_positions.remove(&to);
        new_board.true_count -= 1;
        let (from_row, from_column) = self.pos_to_coor_board_map[&from];
        let (to_row, to_column) = self.pos_to_coor_board_map[&to];
        let to_value = self.data[to - 1];
        if from_column == to_column {
            if from_row < to_row {
                let middle_coor = (from_row + 1, from_column);
                let middle_pos = self.coor_to_pos_board_map[&middle_coor];
                let middle_value = self.data[middle_pos - 1];
                if to_value == false && middle_value == true {
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&middle_coor]);
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row, from_column)]);
                    new_board.data[from - 1] = false;
                    new_board.data[middle_pos - 1] = false;
                    new_board.data[to - 1] = true;

                    Some(new_board)
                } else {
                    None
                }
            } else {
                let middle_coor = (from_row - 1, from_column);
                let middle_pos = self.coor_to_pos_board_map[&middle_coor];
                let middle_value = self.data[middle_pos - 1];
                if to_value == false && middle_value == true {
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row, from_column)]);
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row - 1, from_column)]);
                    new_board.data[from - 1] = false;
                    new_board.data[middle_pos - 1] = false;
                    new_board.data[to - 1] = true;

                    Some(new_board)
                } else {
                    None
                }
            }
        } else if from_row == to_row {
            if from_column < to_column {
                let middle_coor = (from_row, from_column + 1);
                let middle_pos = self.coor_to_pos_board_map[&middle_coor];
                let middle_value = self.data[middle_pos - 1];
                if to_value == false && middle_value == true {
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row, from_column)]);
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row, from_column + 1)]);
                    new_board.data[from - 1] = false;
                    new_board.data[middle_pos - 1] = false;
                    new_board.data[to - 1] = true;

                    Some(new_board)
                } else {
                    None
                }
            } else {
                let middle_coor = (from_row, from_column - 1);
                let middle_pos = self.coor_to_pos_board_map[&middle_coor];
                let middle_value = self.data[middle_pos - 1];
                if to_value == false && middle_value == true {
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row, from_column)]);
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row, from_column - 1)]);
                    new_board.data[from - 1] = false;
                    new_board.data[middle_pos - 1] = false;
                    new_board.data[to - 1] = true;

                    Some(new_board)
                } else {
                    None
                }
            }
        } else {
            if from_row < to_row && from_column < to_column {
                let middle_coor = (from_row + 1, from_column + 1);
                let middle_pos = self.coor_to_pos_board_map[&middle_coor];
                let middle_value = self.data[middle_pos - 1];
                if to_value == false && middle_value == true {
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row, from_column)]);
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row + 1, from_column + 1)]);
                    new_board.data[from - 1] = false;
                    new_board.data[middle_pos - 1] = false;
                    new_board.data[to - 1] = true;

                    Some(new_board)
                } else {
                    None
                }
            } else {
                let middle_coor = (from_row - 1, from_column - 1);
                let middle_pos = self.coor_to_pos_board_map[&middle_coor];
                let middle_value = self.data[middle_pos - 1];
                if to_value == false && middle_value == true {
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row, from_column)]);
                    new_board
                        .empty_positions
                        .insert(self.coor_to_pos_board_map[&(from_row - 1, from_column - 1)]);
                    new_board.data[from - 1] = false;
                    new_board.data[middle_pos - 1] = false;
                    new_board.data[to - 1] = true;

                    Some(new_board)
                } else {
                    None
                }
            }
        }
    }
    //checks to see if the current board state is in a terminal state.
    fn is_terminal(&self) -> bool {
        if self.true_count == 1 {
            true
        } else {
            false
        }
    }
}
//Purpose: to transform a one dimensional array structure position
//         into a two-dimensional structure position.
fn get_board_coors(size: usize, position: usize) -> (usize, usize) {
    let mut prev_number: usize = 1;
    let mut found_number: usize = 1;
    let tmap = create_triangle_map(size);
    let triangle_numbers: Vec<usize> = (1..size + 1).
                                       map(|x| (x * (x + 1)) / 2).collect();
    for each_number in triangle_numbers {
        //if the position is between two triangle numbers return
        //the number associated with the greater of the two.
        if position >= prev_number && position <= each_number {
            found_number = each_number;
            break;
        } else if position > each_number {
            prev_number = each_number;
            continue;
        }
    }
    //obtain the row by getting the number
    //used to obtain the value of found number
    //using the triangle number function.
    //Example if found_number -> 15 then
    //the row will be 4
    let row = tmap[&found_number] - 1;
    let column;
    if row == 0 {
        column = position - prev_number;
    } else {
        column = (position - prev_number) - 1;
    }
    return (row, column);
}

//Creates the mapping between board position in a one-dimensional space 
//representation to the position in the two-dimensional space representation.
fn create_board_position_map(size: usize) -> HashMap<usize, (usize, usize)> {
    let mut board_position_map = HashMap::new();
    for each_number in 1..triangle_number(size) + 1 {
        board_position_map.insert(each_number, get_board_coors(size, each_number));
    }
    return board_position_map;
}

//Same as the function above but maps the two-dimensional space representation
//to the one-dimensional space representation.
fn create_reverse_board_position_map(size: usize) -> HashMap<(usize, usize), usize> {
    let mut reverse_position_map = HashMap::new();
    for each_number in 1..triangle_number(size) + 1 {
        reverse_position_map.insert(get_board_coors(size, each_number), each_number);
    }
    return reverse_position_map;
}
//Maps each triangle number to the corresponding size of the triangle.
//Such that 15 -> 5 where triangle_number(5) -> 15.
fn create_triangle_map(size: usize) -> HashMap<usize, usize> {
    let mut triangle_hash_map = HashMap::new();
    for each_number in 1..size + 1 {
        triangle_hash_map.insert(triangle_number(each_number), each_number);
    }
    return triangle_hash_map;
}

//breadth first search of the state space where the state space contains all
//the corresponding board states linked together with the appropriate move to
//get to the state.
fn search_for_solution(start_board: Board) -> Option<Node> {
    let mut frontier: VecDeque<Node> = VecDeque::<Node>::new();
    let start_node = Node::new(Vec::<(usize, usize)>::new(), start_board);
    frontier.push_back(start_node);
    while !frontier.is_empty() {
        //pop the front of the frontier
        let mut node = frontier.remove(0).unwrap();
        if node.board.is_terminal() {
            //solution found
            return Some(node);
        }
        for (from, to) in node.board.get_actions() {
            let new_state = node.board.action(from, to).unwrap();
            let mut new_move_list = node.move_list.clone();
            //add new move to the move list
            new_move_list.push((from, to));
            frontier.push_back(Node::new(new_move_list, new_state));
        }
    }
    //if we get here no solution was found.
    return None;
}

//entry point in the program
fn main() {
    //obtain the command line arguments
    let args: Vec<String> = env::args().collect();
    //the size of the triangle is the first arguement
    let size = &args[1].parse::<usize>().expect("invalid size parameter");
    //the position of the empty peg slot is the second argument
    let position = &args[2].parse::<usize>().expect("invalid position parameter");
    //create the board_map
    let pos_to_coor_board_map = create_board_position_map(*size);
    let coor_to_pos_board_map = create_reverse_board_position_map(*size);
    let new_board = Board::new(*size, *position, 
                                pos_to_coor_board_map, coor_to_pos_board_map);
    println!("Peg board size {}", size);
    println!("Open position {}", position);
    let found_node = search_for_solution(new_board).unwrap();
    println!("Move list");
    println!("-----------------");
    for action_move in &found_node.move_list {
        let (from, to) = *action_move;
        println!("Move {} to {}", from, to);
    }
}
//end
