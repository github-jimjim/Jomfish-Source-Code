use chess::{Board, Color, MoveGen, Piece};
use crate::engine::engine::Engine;
use crate::engine::utils::piece_from_int;

impl Engine {
    pub fn material_eval(&self, board: Board) -> i32 {
        let mut score = 0;
        for piece_type in 1..=6 {
            let piece = piece_from_int(piece_type as u8);
            let white_count = (board.pieces(piece) & board.color_combined(Color::White)).popcnt() as i32;
            let black_count = (board.pieces(piece) & board.color_combined(Color::Black)).popcnt() as i32;
            score += white_count * self.piece_values[piece_type as usize];
            score -= black_count * self.piece_values[piece_type as usize];
        }
        score
    }

    pub fn position_eval(&self, board: Board) -> i32 {
        let mut score = 0;
        for piece_type in 1..=6 {
            let piece = piece_from_int(piece_type as u8);
            for sq in (board.pieces(piece) & board.color_combined(Color::White)).into_iter() {
                score += self.piece_values[piece_type as usize];
                let sq_index = sq.to_index() as usize;
                let table_index = if sq_index == 0 { 0 } else { (64 - sq_index) % 64 };
                score += self.square_table[piece_type as usize][table_index];
            }
            for sq in (board.pieces(piece) & board.color_combined(Color::Black)).into_iter() {
                score -= self.piece_values[piece_type as usize];
                let sq_index = sq.to_index() as usize;
                score -= self.square_table[piece_type as usize][sq_index];
            }
        }
        score
    }

    pub fn endgame_evaluation(&self, board: Board) -> i32 {
        let base_score = self.position_eval(board);
        if MoveGen::new_legal(&board).next().is_none() {
            if board.checkers().popcnt() == 0 {
                return 0;
            } else {
                if board.side_to_move() == Color::White {
                    return -100000;
                } else {
                    return 100000;
                }
            }
        }
        base_score
    }
}
