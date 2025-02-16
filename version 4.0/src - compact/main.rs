pub mod Jomfish {
pub mod board {
use crate::Jomfish::pieces::{Direction, Square};
use std::fmt::Debug;

pub const PADDING: usize = 2;
pub const BOARD_SIDE: usize = 8 + 2 * PADDING;
pub const BOARD_SIZE: usize = BOARD_SIDE * BOARD_SIDE;

pub const A8: usize = BOARD_SIDE * PADDING + PADDING;
pub const H8: usize = A8 + 7;
pub const A1: usize = A8 + 7 * BOARD_SIDE;
const H1: usize = A1 + 7;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct BoardState {
    pub board: [Square; BOARD_SIZE],
    pub score: i32,
    pub my_castling_rights: (bool, bool),
    pub opponent_castling_rights: (bool, bool),
    pub en_passant_position: Option<usize>,
    pub king_passant_position: Option<usize>,
}

pub fn piece_moves(
    board_state: &BoardState,
    piece_moving: Square,
    start_position: usize,
) -> Vec<usize> {
    let mut reachable_squares: Vec<usize> = Vec::with_capacity(20);
    for move_direction in piece_moving.moves() {
        for end_position in (1..).map(|k| (start_position as i32 + move_direction * k) as usize) {
            let destination_square = board_state.board[end_position];
            if destination_square == Square::Wall || destination_square.is_my_piece() {
                break;
            }

            if piece_moving == Square::MyPawn {
                if (*move_direction == Direction::NORTH
                    || *move_direction == Direction::NORTH + Direction::NORTH)
                    && destination_square != Square::Empty
                {
                    break;
                }
                if (*move_direction == Direction::NORTH + Direction::WEST
                    || *move_direction == Direction::NORTH + Direction::EAST)
                    && destination_square == Square::Empty
                    && board_state.en_passant_position != Some(end_position)
                    && board_state.king_passant_position != Some(end_position)
                {
                    break;
                }
                if *move_direction == Direction::NORTH + Direction::NORTH
                    && (start_position < (A1 as i32 + Direction::NORTH) as usize
                        || board_state.board[(start_position as i32 + Direction::NORTH) as usize]
                            != Square::Empty)
                {
                    break;
                }
            }

            reachable_squares.push(end_position);

            if piece_moving == Square::MyPawn
                || piece_moving == Square::MyKnight
                || piece_moving == Square::MyKing
            {
                break;
            }

            if destination_square != Square::Empty {
                break;
            }
        }
    }
    reachable_squares
}

pub fn gen_moves(board_state: &BoardState) -> Vec<(usize, usize)> {
    let mut moves: Vec<(usize, usize)> = Vec::with_capacity(42);
    for (start_position, start_square) in board_state.board.iter().enumerate() {
        if !start_square.is_my_piece() {
            continue;
        }
        let piece_moving = start_square;
        for end_position in piece_moves(board_state, *piece_moving, start_position) {
            moves.push((start_position, end_position));
            if start_position == A1
                && board_state.board[(end_position as i32 + Direction::EAST) as usize]
                    == Square::MyKing
                && board_state.my_castling_rights.0
            {
                moves.push((
                    (end_position as i32 + Direction::EAST) as usize,
                    (end_position as i32 + Direction::WEST) as usize,
                ))
            } else if start_position == H1
                && board_state.board[(end_position as i32 + Direction::WEST) as usize]
                    == Square::MyKing
                && board_state.my_castling_rights.1
            {
                moves.push((
                    (end_position as i32 + Direction::WEST) as usize,
                    (end_position as i32 + Direction::EAST) as usize,
                ))
            }
        }
    }
    moves
}

pub fn rotate(board_state: &mut BoardState) {
    let total_padding = PADDING * BOARD_SIDE + PADDING;
    for coordinate in total_padding..(BOARD_SIZE / 2) {
        let old_val = board_state.board[coordinate];
        board_state.board[coordinate] = board_state.board[BOARD_SIZE - 1 - coordinate].swap_color();
        board_state.board[BOARD_SIZE - 1 - coordinate] = old_val.swap_color();
    }
    board_state.score = -board_state.score;
    std::mem::swap(
        &mut board_state.my_castling_rights,
        &mut board_state.opponent_castling_rights,
    );
    board_state.en_passant_position = board_state
        .en_passant_position
        .map(|ep| BOARD_SIZE - 1 - ep);
    board_state.king_passant_position = board_state
        .king_passant_position
        .map(|kp| BOARD_SIZE - 1 - kp);
}

pub fn nullmove(board_state: &BoardState) -> BoardState {
    let mut new_board = [Square::Empty; BOARD_SIZE];
    for (coordinate, square) in new_board.iter_mut().enumerate() {
        *square = board_state.board[BOARD_SIZE - 1 - coordinate].swap_color();
    }
    BoardState {
        board: new_board,
        score: -board_state.score,
        my_castling_rights: board_state.opponent_castling_rights,
        opponent_castling_rights: board_state.my_castling_rights,
        en_passant_position: None,
        king_passant_position: None,
    }
}

pub fn after_move(board_state: &BoardState, move_: &(usize, usize)) -> BoardState {
    let (start_position, end_position) = *move_;
    let start_square = board_state.board[start_position];
    let mut new_board = board_state.board;
    let mut my_castling_rights = board_state.my_castling_rights;
    let mut opponent_castling_rights = board_state.opponent_castling_rights;
    let mut en_passant_position = None;
    let mut king_passant_position = None;

    new_board[end_position] = start_square;
    new_board[start_position] = Square::Empty;

    if start_position == A1 {
        my_castling_rights = (false, my_castling_rights.1)
    }
    if start_position == H1 {
        my_castling_rights = (my_castling_rights.0, false)
    }
    if end_position == A8 {
        opponent_castling_rights = (opponent_castling_rights.0, false)
    }
    if end_position == H8 {
        opponent_castling_rights = (false, opponent_castling_rights.1)
    }

    if start_square == Square::MyKing {
        my_castling_rights = (false, false);
        if (start_position as i32 - end_position as i32).abs() == 2 {
            let final_rook_position: usize = (start_position + end_position) / 2;
            new_board[final_rook_position] = Square::MyRook;
            king_passant_position = Some(final_rook_position);
            if start_position > end_position {
                new_board[A1] = Square::Empty;
            } else {
                new_board[H1] = Square::Empty;
            }
        }
    }

    if start_square == Square::MyPawn {
        let move_type = end_position as i32 - start_position as i32;
        if (A8 <= end_position) && (end_position <= H8) {
            new_board[end_position] = Square::MyQueen
        } else if move_type == 2 * Direction::NORTH {
            en_passant_position = Some((start_position as i32 + Direction::NORTH) as usize)
        }

        if board_state.en_passant_position == Some(end_position) {
            new_board[end_position + Direction::SOUTH as usize] = Square::Empty;
        }
    }

    let mut new_board_state = BoardState {
        board: new_board,
        score: board_state.score + move_value(board_state, &move_),
        my_castling_rights,
        opponent_castling_rights,
        king_passant_position,
        en_passant_position,
    };
    rotate(&mut new_board_state);
    new_board_state
}

pub fn can_check(board_state: &BoardState, move_: &(usize, usize)) -> bool {
    let (start_position, end_position) = *move_;
    let moved_piece = board_state.board[start_position];
    if !moved_piece.is_my_piece() {
        panic!();
    }
    for reachable_square in piece_moves(board_state, moved_piece, end_position) {
        if board_state.board[reachable_square] == Square::OpponentKing {
            return true;
        }
    }
    false
}

pub fn move_value(board_state: &BoardState, move_: &(usize, usize)) -> i32 {
    let (start_position, end_position) = *move_;
    let moving_piece = board_state.board[start_position];
    if !moving_piece.is_my_piece() {
        panic!();
    }

    let mut temp_score =
        moving_piece.midgame_value(end_position) - moving_piece.midgame_value(start_position);

    if board_state.board[end_position].is_opponent_piece() {
        temp_score += board_state.board[end_position]
            .swap_color()
            .midgame_value(BOARD_SIZE - 1 - end_position);
    }

    match board_state.king_passant_position {
        None => {}
        Some(position) => {
            if (end_position as i32 - position as i32).abs() < 2 {
                temp_score += Square::MyKing.midgame_value(BOARD_SIZE - 1 - end_position);
            }
        }
    }

    match moving_piece {
        Square::MyKing => {
            if (end_position as i32 - start_position as i32).abs() == 2 {
                temp_score += Square::MyRook.midgame_value((start_position + end_position) / 2);
                temp_score -= Square::MyRook.midgame_value(if end_position < start_position {
                    A1
                } else {
                    H1
                });
            }
        }
        Square::MyPawn => {
            if A8 <= end_position && end_position <= H8 {
                temp_score += Square::MyQueen.midgame_value(end_position)
                    - Square::MyPawn.midgame_value(end_position)
            } else if board_state.en_passant_position == Some(end_position) {
                temp_score +=
                    Square::MyPawn.midgame_value(BOARD_SIZE - 1 - (end_position + BOARD_SIDE))
            }
        }
        _ => {}
    }
    temp_score
}

pub fn static_score(board: [Square; BOARD_SIZE]) -> i32 {
    board
        .iter()
        .enumerate()
        .map(|(index, piece)| {
            if piece.is_my_piece() {
                piece.midgame_value(index)
            } else if piece.is_opponent_piece() {
                -piece.swap_color().midgame_value(BOARD_SIZE - 1 - index)
            } else {
                0
            }
        })
        .sum()
}

const INITIAL_BOARD: [Square; BOARD_SIZE] = [
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::OpponentRook,
    Square::OpponentKnight,
    Square::OpponentBishop,
    Square::OpponentQueen,
    Square::OpponentKing,
    Square::OpponentBishop,
    Square::OpponentKnight,
    Square::OpponentRook,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::OpponentPawn,
    Square::OpponentPawn,
    Square::OpponentPawn,
    Square::OpponentPawn,
    Square::OpponentPawn,
    Square::OpponentPawn,
    Square::OpponentPawn,
    Square::OpponentPawn,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Empty,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::MyPawn,
    Square::MyPawn,
    Square::MyPawn,
    Square::MyPawn,
    Square::MyPawn,
    Square::MyPawn,
    Square::MyPawn,
    Square::MyPawn,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::MyRook,
    Square::MyKnight,
    Square::MyBishop,
    Square::MyQueen,
    Square::MyKing,
    Square::MyBishop,
    Square::MyKnight,
    Square::MyRook,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
    Square::Wall,
];

pub const INITIAL_BOARD_STATE: BoardState = BoardState {
    board: INITIAL_BOARD,
    score: 0,
    my_castling_rights: (true, true),
    opponent_castling_rights: (true, true),
    en_passant_position: None,
    king_passant_position: None,
};

}
pub mod pieces {
use crate::Jomfish::board::{BOARD_SIDE, BOARD_SIZE, PADDING};

pub struct Direction {}

impl Direction {
    pub const NORTH: i32 = -(BOARD_SIDE as i32);
    pub const EAST: i32 = 1;
    pub const SOUTH: i32 = BOARD_SIDE as i32;
    pub const WEST: i32 = -1;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Square {
    MyPawn = 0x01,
    MyKnight = 0x02,
    MyBishop = 0x03,
    MyRook = 0x04,
    MyQueen = 0x05,
    MyKing = 0x06,
    OpponentPawn = 0x11,
    OpponentKnight = 0x12,
    OpponentBishop = 0x13,
    OpponentRook = 0x14,
    OpponentQueen = 0x15,
    OpponentKing = 0x16,
    Empty = 0xFE,
    Wall = 0xFF,
}

impl Square {
    pub fn is_my_piece(self) -> bool {
        matches!(
            self,
            Square::MyPawn
                | Square::MyKing
                | Square::MyRook
                | Square::MyKnight
                | Square::MyBishop
                | Square::MyQueen
        )
    }

    pub fn is_opponent_piece(self) -> bool {
        matches!(
            self,
            Square::OpponentPawn
                | Square::OpponentKing
                | Square::OpponentRook
                | Square::OpponentKnight
                | Square::OpponentBishop
                | Square::OpponentQueen
        )
    }

    pub fn swap_color(self) -> Square {
        match self {
            Square::Empty => Square::Empty,
            Square::Wall => Square::Wall,
            Square::MyPawn => Square::OpponentPawn,
            Square::MyKing => Square::OpponentKing,
            Square::MyRook => Square::OpponentRook,
            Square::MyKnight => Square::OpponentKnight,
            Square::MyBishop => Square::OpponentBishop,
            Square::MyQueen => Square::OpponentQueen,
            Square::OpponentPawn => Square::MyPawn,
            Square::OpponentKing => Square::MyKing,
            Square::OpponentRook => Square::MyRook,
            Square::OpponentKnight => Square::MyKnight,
            Square::OpponentBishop => Square::MyBishop,
            Square::OpponentQueen => Square::MyQueen,
        }
    }

    pub fn moves(self) -> &'static [i32] {
        match self {
            Square::MyPawn => &[
                Direction::NORTH,
                Direction::NORTH + Direction::NORTH,
                Direction::NORTH + Direction::WEST,
                Direction::NORTH + Direction::EAST,
            ],
            Square::MyKnight => &[
                Direction::NORTH + Direction::NORTH + Direction::EAST,
                Direction::NORTH + Direction::NORTH + Direction::WEST,
                Direction::WEST + Direction::WEST + Direction::NORTH,
                Direction::WEST + Direction::WEST + Direction::SOUTH,
                Direction::SOUTH + Direction::SOUTH + Direction::WEST,
                Direction::SOUTH + Direction::SOUTH + Direction::EAST,
                Direction::EAST + Direction::EAST + Direction::SOUTH,
                Direction::EAST + Direction::EAST + Direction::NORTH,
            ],
            Square::MyBishop => &[
                Direction::NORTH + Direction::EAST,
                Direction::NORTH + Direction::WEST,
                Direction::WEST + Direction::SOUTH,
                Direction::SOUTH + Direction::EAST,
            ],
            Square::MyRook => &[
                Direction::NORTH,
                Direction::WEST,
                Direction::SOUTH,
                Direction::EAST,
            ],
            Square::MyQueen | Square::MyKing => &[
                Direction::NORTH,
                Direction::WEST,
                Direction::SOUTH,
                Direction::EAST,
                Direction::NORTH + Direction::EAST,
                Direction::NORTH + Direction::WEST,
                Direction::WEST + Direction::SOUTH,
                Direction::SOUTH + Direction::EAST,
            ],
            _ => panic!(),
        }
    }

    pub fn midgame_value(self, position: usize) -> i32 {
        debug_assert!(
            position >= BOARD_SIDE * PADDING + PADDING
                && position < BOARD_SIZE - BOARD_SIDE * PADDING - PADDING
                && position % BOARD_SIDE >= PADDING
                && position % BOARD_SIDE < BOARD_SIDE - PADDING
        );

        let piece_value = match self {
            Square::MyPawn => 136,
            Square::MyKnight => 782,
            Square::MyBishop => 830,
            Square::MyRook => 1289,
            Square::MyQueen => 2529,
            Square::MyKing => 32000,
            _ => panic!(),
        };

        let piece_position_value = match self {
            Square::MyPawn => &[
                0, 0, 0, 0, 0, 0, 0, 0, 15, 31, 20, 14, 23, 11, 37, 24, -1, -3, 15, 26, 1, 10, -7,
                -9, 8, -1, -5, 13, 24, 11, -10, 3, -9, -18, 8, 32, 43, 25, -4, -16, -9, -13, -40,
                22, 26, -40, 1, -22, 2, 0, 15, 3, 11, 22, 11, -1, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            Square::MyKnight => &[
                -200, -80, -53, -32, -32, -53, -80, -200, -67, -21, 6, 37, 37, 6, -21, -67, -11,
                28, 63, 55, 55, 63, 28, -11, -29, 13, 42, 52, 52, 42, 13, -29, -28, 5, 41, 47, 47,
                41, 5, -28, -64, -20, 4, 19, 19, 4, -20, -64, -79, -39, -24, -9, -9, -24, -39, -79,
                -169, -96, -80, -79, -79, -80, -96, -169,
            ],
            Square::MyBishop => &[
                -48, -3, -12, -25, -25, -12, -3, -48, -21, -19, 10, -6, -6, 10, -19, -21, -17, 4,
                -1, 8, 8, -1, 4, -17, -7, 30, 23, 28, 28, 23, 30, -7, 1, 8, 26, 37, 37, 26, 8, 1,
                -8, 24, -3, 15, 15, -3, 24, -8, -18, 7, 14, 3, 3, 14, 7, -18, -44, -4, -11, -28,
                -28, -11, -4, -44,
            ],
            Square::MyRook => &[
                -22, -24, -6, 4, 4, -6, -24, -22, -8, 6, 10, 12, 12, 10, 6, -8, -24, -4, 4, 10, 10,
                4, -4, -24, -24, -12, -1, 6, 6, -1, -12, -24, -13, -5, -4, -6, -6, -4, -5, -13,
                -21, -7, 3, -1, -1, 3, -7, -21, -18, -10, -5, 9, 9, -5, -10, -18, -24, -13, -7, 2,
                2, -7, -13, -24,
            ],
            Square::MyQueen => &[
                -2, -2, 1, -2, -2, 1, -2, -2, -5, 6, 10, 8, 8, 10, 6, -5, -4, 10, 6, 8, 8, 6, 10,
                -4, 0, 14, 12, 5, 5, 12, 14, 0, 4, 5, 9, 8, 8, 9, 5, 4, -3, 6, 13, 7, 7, 13, 6, -3,
                -3, 5, 8, 12, 12, 8, 5, -3, 3, -5, -5, 4, 4, -5, -5, 3,
            ],
            Square::MyKing => &[
                6, 8, 4, 0, 0, 4, 8, 6, 8, 12, 6, 2, 2, 6, 12, 8, 12, 15, 8, 3, 3, 8, 15, 12, 14,
                17, 11, 6, 6, 11, 17, 15, 16, 19, 13, 10, 10, 13, 19, 16, 19, 25, 16, 12, 12, 16,
                25, 19, 27, 30, 24, 18, 18, 24, 30, 27, 27, 32, 27, 19, 19, 27, 32, 27,
            ],
            _ => &[0; 64],
        };
        let real_position = position - PADDING * BOARD_SIDE;
        let row_number = real_position / BOARD_SIDE;
        piece_value + piece_position_value[real_position - PADDING * (2 * row_number + 1)]
    }
}

}
pub mod render {
use crate::Jomfish::board::{A1, A8, BOARD_SIDE};

pub fn parse_move(move_: &str) -> (usize, usize) {
    let from = parse_coordinates(&move_[..2]);
    let to = parse_coordinates(&move_[2..]);
    (from, to)
}

fn parse_coordinates(coordinates: &str) -> usize {
    let mut chars = coordinates.chars();
    let file = chars.next().expect("Failed to parse coordinates");
    let rank = chars.next().expect("Failed to parse coordinates");
    A1 + (file as i32 - 'a' as i32) as usize - BOARD_SIDE * ((rank as i32 - '1' as i32) as usize)
}

pub fn render_move(mv: &(usize, usize)) -> String {
    render_coordinates(mv.0) + &render_coordinates(mv.1)
}

fn render_coordinates(position: usize) -> String {
    let rank = b'8' - ((position - A8) as u8 / BOARD_SIDE as u8);
    let file = (position - A8) as u8 % BOARD_SIDE as u8 + b'a';
    [file as char, rank as char].iter().collect()
}

}
pub mod search {
use log::info;
use std::cmp::max;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::Jomfish::board::{after_move, can_check, gen_moves, move_value, nullmove, BoardState};
use crate::Jomfish::pieces::Square;

pub const MATE_UPPER: i32 = 32_000 + 8 * 2529;
pub const MATE_LOWER: i32 = 32_000 - 8 * 2529;
const TRANSPOSITION_TABLE_SIZE: usize = 1_000_000;
const QUIESCENCE_SEARCH_LIMIT: i32 = 130;
const EVAL_ROUGHNESS: i32 = 10;
const STOP_SEARCH: i32 = MATE_UPPER * 101;

#[derive(Clone, Copy)]
pub struct Entry {
    lower: i32,
    upper: i32,
}

const DEFAULT_ENTRY: Entry = Entry {
    lower: -MATE_UPPER,
    upper: MATE_UPPER,
};

pub struct Searcher {
    pub score_transposition_table: HashMap<(BoardState, i32, bool), Entry>,
    pub move_transposition_table: HashMap<BoardState, (usize, usize)>,
    pub nodes: u32,
    now: Instant,
    duration: Duration,
}

impl Default for Searcher {
    fn default() -> Self {
        Searcher {
            score_transposition_table: HashMap::with_capacity(TRANSPOSITION_TABLE_SIZE),
            move_transposition_table: HashMap::with_capacity(TRANSPOSITION_TABLE_SIZE),
            nodes: 0,
            now: Instant::now(),
            duration: Duration::new(4, 0),
        }
    }
}

impl Searcher {
    fn bound(&mut self, board_state: &BoardState, gamma: i32, depth: i32, root: bool) -> i32 {
        self.nodes += 1;

        if board_state.score <= -MATE_LOWER {
            return -MATE_UPPER;
        }

        let entry = *self
            .score_transposition_table
            .get(&(*board_state, max(depth, 0), root))
            .unwrap_or(&DEFAULT_ENTRY);

        if entry.lower >= gamma
            && (!root || self.move_transposition_table.get(board_state).is_some())
        {
            return entry.lower;
        } else if entry.upper < gamma {
            return entry.upper;
        }

        if self.now.elapsed() > self.duration {
            return STOP_SEARCH;
        }

        let mut best = -MATE_UPPER;
        if depth > 0
            && !root
            && (board_state.board.iter().any(|&s| {
                matches!(
                    s,
                    Square::MyRook | Square::MyKnight | Square::MyBishop | Square::MyQueen
                )
            }))
        {
            let score = -self.bound(&nullmove(board_state), 1 - gamma, depth - 3, false);
            if score == -STOP_SEARCH {
                return STOP_SEARCH;
            }
            best = std::cmp::max(best, score);
        } else if depth <= 0 {
            let score = board_state.score;
            best = std::cmp::max(best, score);
        }

        if best <= gamma {
            if let Some(killer_move) = self.move_transposition_table.get(board_state).copied() {
                if depth > 0 || move_value(board_state, &killer_move) >= QUIESCENCE_SEARCH_LIMIT {
                    let score = -self.bound(
                        &after_move(board_state, &killer_move),
                        1 - gamma,
                        depth - 1,
                        false,
                    );
                    if score == -STOP_SEARCH {
                        return STOP_SEARCH;
                    }
                    best = std::cmp::max(best, score);
                }
            }
        }

        if best < gamma {
            let others = gen_moves(board_state);
            let check_bonus = |m| {
                if can_check(board_state, m) {
                    QUIESCENCE_SEARCH_LIMIT / 2
                } else {
                    0
                }
            };
            let mut move_vals: Vec<_> = others
                .iter()
                .map(|m| (-move_value(board_state, m) - check_bonus(m), m))
                .collect();
            move_vals.sort_unstable();
            for (val, m) in move_vals {
                if depth > 0
                    || (-val >= QUIESCENCE_SEARCH_LIMIT && (board_state.score - val > best))
                {
                    let score =
                        -self.bound(&after_move(board_state, m), 1 - gamma, depth - 1, false);
                    if score == -STOP_SEARCH {
                        return STOP_SEARCH;
                    }
                    best = std::cmp::max(best, score);
                    if best >= gamma {
                        if self.move_transposition_table.len() >= TRANSPOSITION_TABLE_SIZE {
                            self.move_transposition_table.clear();
                        }
                        self.move_transposition_table.insert(*board_state, *m);
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if best < gamma && best < 0 && depth > 0 {
            let is_dead = |pos: BoardState| {
                gen_moves(&pos)
                    .iter()
                    .any(|m| move_value(&pos, m) >= MATE_LOWER)
            };
            if gen_moves(board_state)
                .iter()
                .all(|m| is_dead(after_move(board_state, m)))
            {
                let in_check = is_dead(nullmove(board_state));
                best = if in_check { -MATE_UPPER } else { 0 };
            }
        }

        if self.score_transposition_table.len() >= TRANSPOSITION_TABLE_SIZE {
            self.score_transposition_table.clear();
        }
        if best >= gamma {
            self.score_transposition_table.insert(
                (*board_state, depth, root),
                Entry {
                    lower: best,
                    upper: entry.upper,
                },
            );
        } else if best < gamma {
            self.score_transposition_table.insert(
                (*board_state, depth, root),
                Entry {
                    lower: entry.lower,
                    upper: best,
                },
            );
        }

        best
    }

    pub fn search(
        &mut self,
        board_state: BoardState,
        movetime: Option<Duration>,
        max_depth: Option<i32>,
    ) -> ((usize, usize), i32, i32) {
        self.nodes = 0;
        self.now = Instant::now();
        self.duration = movetime.unwrap_or(Duration::new(10000, 0));
        let max_depth = max_depth.unwrap_or(99);
        let mut last_move = ((0, 0), 0, 0);

        for depth in 1..=max_depth {
            let mut lower = -MATE_UPPER;
            let mut upper = MATE_UPPER;
            while lower < upper - EVAL_ROUGHNESS {
                let gamma = (lower + upper + 1) / 2;
                let score = self.bound(&board_state, gamma, depth, true);
                if score == STOP_SEARCH {
                    lower = STOP_SEARCH;
                    break;
                }
                if score >= gamma {
                    lower = score;
                } else {
                    upper = score;
                }
            }
            if lower == STOP_SEARCH {
                break;
            }
            let score = self.bound(&board_state, lower, depth, true);
            if score == STOP_SEARCH {
                break;
            }
            let elapsed = self.now.elapsed().as_secs_f64();
            let nps = self.nodes as f64 / elapsed.max(0.001);
            info!(
                "Reached depth {: <2} score {: <5} nodes {: <7} time {:?} nps {:.2}",
                depth,
                score,
                self.nodes,
                self.now.elapsed(),
                nps
            );
            println!(
                "info depth {: <2} score {: <5} nodes {: <7} time {:?} nps {:.2}",
                depth,
                score,
                self.nodes,
                self.now.elapsed(),
                nps
            );

            last_move = (
                *self
                    .move_transposition_table
                    .get(&board_state)
                    .expect("move not in table"),
                self.score_transposition_table
                    .get(&(board_state, depth, true))
                    .expect("score not in table")
                    .lower,
                depth,
            );

            if self.now.elapsed() > self.duration || score > MATE_LOWER {
                break;
            }
        }

        let elapsed = self.now.elapsed().as_secs_f64();
        let nps = self.nodes as f64 / elapsed.max(0.001);
        info!(
            "Search complete: {} nodes in {:.3} seconds ({:.2} nps)",
            self.nodes, elapsed, nps
        );
        println!("info nps: {:.2}", nps);

        last_move
    }

    pub fn set_eval_to_zero(&mut self, board_state: &BoardState) {
        for depth in 1..30 {
            self.score_transposition_table
                .insert((*board_state, depth, false), Entry { lower: 0, upper: 0 });
        }
    }
}

}

}
use log::{info, trace, warn};
use std::time::Duration;

use Jomfish::board::{after_move, gen_moves, A8, BOARD_SIZE, H8, INITIAL_BOARD_STATE};
use Jomfish::pieces::Square;
use Jomfish::render::{parse_move, render_move};
use Jomfish::search::Searcher;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    println!("id name Jomfish 4.0");
    println!("id author Jimmy Luong");
	println!("option name move_overhead type spin default 50 min 0 max 5000");
    println!("uciok");

    let mut board_state = INITIAL_BOARD_STATE;
    let mut am_black = false;
    let mut move_overhead = Duration::from_millis(50);

    loop {
        let next_command = read_line();
        trace!("Received command: {}", next_command);

        let tokens: Vec<&str> = next_command.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "quit" => return,
            "uci" => {
                println!("id name Jomfish 4.0");
                println!("id author Jimmy Luong");
                println!("option name move_overhead type spin default 50 min 0 max 5000");
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "setoption" => {
                let mut option_name = String::new();
                let mut option_value = "";
                let mut i = 1;
                while i < tokens.len() {
                    if tokens[i] == "name" {
                        i += 1;
                        let mut name_parts = Vec::new();
                        while i < tokens.len() && tokens[i] != "value" {
                            name_parts.push(tokens[i]);
                            i += 1;
                        }
                        option_name = name_parts.join(" ");
                    } else if tokens[i] == "value" {
                        i += 1;
                        if i < tokens.len() {
                            option_value = tokens[i];
                        }
                        break;
                    } else {
                        i += 1;
                    }
                }
                if option_name == "move_overhead" {
                    if let Ok(ms) = option_value.parse::<u64>() {
                        move_overhead = Duration::from_millis(ms);
                        info!("Set Move Overhead to {} ms", ms);
                        info!("Current Move Overhead: {:?}", move_overhead);
                    } else {
                        warn!("Invalid Move Overhead value: {}", option_value);
                    }
                }
            }
            "ucinewgame" => board_state = INITIAL_BOARD_STATE,
            "position" => {
                info!("loading moves");
                if tokens.len() >= 2 && tokens[1] == "startpos" {
                    board_state = INITIAL_BOARD_STATE;
                    am_black = false;

                    if tokens.len() > 3 && tokens[2] == "moves" {
                        for move_ in tokens.iter().skip(3) {
                            let mut parsed_move = parse_move(move_);
                            if am_black {
                                parsed_move.0 = BOARD_SIZE - 1 - parsed_move.0;
                                parsed_move.1 = BOARD_SIZE - 1 - parsed_move.1;
                            }
                            if !gen_moves(&board_state).contains(&parsed_move) {
                                warn!(
                                    "Trying to make an illegal move {:?}, will probably fail",
                                    parsed_move
                                );
                            }
                            board_state = after_move(&board_state, &parsed_move);
                            am_black = !am_black;
                        }
                    }
                } else {
                    warn!("UNKNOWN FORMAT!");
                    println!("info string Unknown position format");
                }
            }
            "go" => {
                let mut searcher = Searcher::default();
                let mut movetime: Option<Duration> = None;
                let mut max_depth: Option<i32> = None;

                if let Some(index) = tokens.iter().position(|&t| t == "depth") {
                    if let Some(&depth_str) = tokens.get(index + 1) {
                        if let Ok(d) = depth_str.parse::<i32>() {
                            max_depth = Some(d);
                            movetime = None;
                        } else {
                            warn!("Invalid depth value: {}", depth_str);
                        }
                    }
                }

                if max_depth.is_none() {
                    if let Some(index) = tokens.iter().position(|&t| t == "movetime") {
                        if let Some(&time_str) = tokens.get(index + 1) {
                            if let Ok(ms) = time_str.parse::<u64>() {
                                let requested_time = Duration::from_millis(ms);
                                let search_time = if requested_time > move_overhead {
                                    requested_time - move_overhead
                                } else {
                                    requested_time
                                };
                                movetime = Some(search_time);
                            } else {
                                warn!("Invalid movetime value: {}", time_str);
                            }
                        }
                    }
                }

                let (mut top_move, _score, _depth) =
                    searcher.search(board_state, movetime, max_depth);
                let is_promotion = (A8 <= top_move.1 && top_move.1 <= H8)
                    && board_state.board[top_move.0] == Square::MyPawn;
                if am_black {
                    top_move.0 = BOARD_SIZE - 1 - top_move.0;
                    top_move.1 = BOARD_SIZE - 1 - top_move.1;
                };

                if is_promotion {
                    println!("bestmove {}q", render_move(&top_move));
                } else {
                    println!("bestmove {}", render_move(&top_move));
                }

                info!("Sending bestmove {}", render_move(&top_move));
                info!(
                    "Searched {} nodes, reached depth {}, estimate score {}, tables at {} and {}",
                    searcher.nodes,
                    _depth,
                    _score,
                    searcher.move_transposition_table.len(),
                    searcher.score_transposition_table.len()
                );
            }

            _ => {
                warn!("UNKNOWN COMMAND {}", next_command);
                println!("info string Unknown command: {}", next_command);
            }
        }
    }
}

