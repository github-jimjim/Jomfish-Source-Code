use std::cmp::{max, min};
use chess::{Board, ChessMove, MoveGen, Color};
use crate::engine::engine::Engine;
use crate::engine::evaluation::*;

impl Engine {
    pub fn minimax(&mut self, board: Board, depth: u32, maximiser: bool) -> (Option<ChessMove>, i32) {
        if depth == 0 {
            return (None, self.position_eval(board));
        }
        let mut best_move = None;
        if maximiser {
            let mut best_score = -1000000;
            for mv in MoveGen::new_legal(&board) {
                self.leaves_reached += 1;
                let new_board = board.make_move_new(mv);
                let (_child_move, score) = self.minimax(new_board, depth - 1, false);
                if score > best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
            }
            (best_move, best_score)
        } else {
            let mut best_score = 1000000;
            for mv in MoveGen::new_legal(&board) {
                self.leaves_reached += 1;
                let new_board = board.make_move_new(mv);
                let (_child_move, score) = self.minimax(new_board, depth - 1, true);
                if score < best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
            }
            (best_move, best_score)
        }
    }

    pub fn alpha_beta(&mut self, board: Board, depth: u32, mut alpha: i32, mut beta: i32, maximiser: bool) -> (Option<ChessMove>, i32) {
        if depth == 0 {
            return (None, self.endgame_evaluation(board));
        }
        
        let mut legal_moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();
        if legal_moves.is_empty() {
            return (None, self.endgame_evaluation(board));
        }
        
        legal_moves.sort_by_key(|mv| {
            let new_board = board.make_move_new(*mv);
            if MoveGen::new_legal(&new_board).next().is_none() && new_board.checkers().popcnt() == 0 {
                1  
            } else {
                0
            }
        });

        let mut best_move = None;
        if maximiser {
            let mut best_score = -10000000;
            for mv in legal_moves {
                let new_board = board.make_move_new(mv);
                let (_child_move, score) = self.alpha_beta(new_board, depth - 1, alpha, beta, false);
                if score > best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
                alpha = max(alpha, best_score);
                if beta <= alpha {
                    break;
                }
            }
            (best_move, best_score)
        } else {
            let mut best_score = 10000000;
            for mv in legal_moves {
                let new_board = board.make_move_new(mv);
                let (_child_move, score) = self.alpha_beta(new_board, depth - 1, alpha, beta, true);
                if score < best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
                beta = min(beta, best_score);
                if beta <= alpha {
                    break;
                }
            }
            (best_move, best_score)
        }
    }

    pub fn iterative_deepening(&mut self, max_depth: u32) -> Option<ChessMove> {
        let mut best_move = None;
        for depth in 1..=max_depth {
            println!("Iteration depth {}", depth);
            let (mv, _score) = self.alpha_beta(
                self.board,
                depth,
                -10000001,
                10000001,
                self.board.side_to_move() == Color::White,
            );
            if mv.is_some() {
                best_move = mv;
            }
        }
        best_move
    }
}
