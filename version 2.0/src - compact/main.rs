use chess::{Board, ChessMove, Color, File, MoveGen, Piece, Rank, Square};
use rand::seq::IteratorRandom;
use std::cmp::{max, min};
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;

fn piece_from_int(piece_type: u8) -> Piece {
    match piece_type {
        1 => Piece::Pawn,
        2 => Piece::Knight,
        3 => Piece::Bishop,
        4 => Piece::Rook,
        5 => Piece::Queen,
        6 => Piece::King,
        _ => panic!("Ungültiger Piece-Typ: {}", piece_type),
    }
}

fn parse_uci_move(uci: &str) -> Option<ChessMove> {
    if uci.len() < 4 {
        return None;
    }
    let bytes = uci.as_bytes();
    let file_from = bytes[0] as char;
    let rank_from = bytes[1] as char;
    let file_to = bytes[2] as char;
    let rank_to = bytes[3] as char;

    let source = Square::make_square(
        Rank::from_index((rank_from as u8 - b'1') as usize),
        File::from_index((file_from as u8 - b'a') as usize),
    );
    let dest = Square::make_square(
        Rank::from_index((rank_to as u8 - b'1') as usize),
        File::from_index((file_to as u8 - b'a') as usize),
    );
    let promotion = if uci.len() == 5 {
        let prom_char = bytes[4] as char;
        match prom_char.to_ascii_lowercase() {
            'q' => Some(Piece::Queen),
            'r' => Some(Piece::Rook),
            'b' => Some(Piece::Bishop),
            'n' => Some(Piece::Knight),
            _ => None,
        }
    } else {
        None
    };
    Some(ChessMove::new(source, dest, promotion))
}

pub struct Engine {
    pub board: Board,
    pub max_depth: u32,
    piece_values: [i32; 7],
    square_table: [Vec<i32>; 7],
    leaves_reached: u64,
}

impl Engine {
    pub fn new(fen: &str) -> Self {
        let board = Board::from_fen(fen.to_string()).unwrap_or_else(|| Board::default());
        let max_depth = 60;
        let piece_values = [0, 100, 310, 300, 500, 900, 99999];

        let square_table_bauer = vec![
              0,   0,   0,   0,   0,   0,   0,   0,
             50,  50,  50,  50,  50,  50,  50,  50,
             10,  10,  20,  30,  30,  20,  10,  10,
              5,   5,  10,  25,  25,  10,   5,   5,
              0,   0,   0,  20,  20,   0,   0,   0,
              5,  -5, -10,   0,   0, -10,  -5,   5,
              5,  10,  10, -20, -20,  10,  10,   5,
              0,   0,   0,   0,   0,   0,   0,   0,
        ];
        let square_table_springer = vec![
            -50, -40, -30, -30, -30, -30, -40, -50,
            -40, -20,   0,   0,   0,   0, -20, -40,
            -30,   0,  10,  15,  15,  10,   0, -30,
            -30,   5,  15,  20,  20,  15,   5, -30,
            -30,   0,  15,  20,  20,  15,   0, -30,
            -30,   5,  10,  15,  15,  10,   5, -30,
            -40, -20,   0,   5,   5,   0, -20, -40,
            -50, -40, -30, -30, -30, -30, -40, -50,
        ];
        let square_table_laeufer = vec![
            -20, -10, -10, -10, -10, -10, -10, -20,
            -10,   0,   0,   0,   0,   0,   0, -10,
            -10,   0,   5,  10,  10,   5,   0, -10,
            -10,   5,   5,  10,  10,   5,   5, -10,
            -10,   0,  10,  10,  10,  10,   0, -10,
            -10,  10,  10,  10,  10,  10,  10, -10,
            -10,   5,   0,   0,   0,   0,   5, -10,
            -20, -10, -10, -10, -10, -10, -10, -20,
        ];
        let square_table_turm = vec![
              0,  0,  0,  0,  0,  0,  0,  0,
              5, 10, 10, 10, 10, 10, 10,  5,
             -5,  0,  0,  0,  0,  0,  0, -5,
             -5,  0,  0,  0,  0,  0,  0, -5,
             -5,  0,  0,  0,  0,  0,  0, -5,
             -5,  0,  0,  0,  0,  0,  0, -5,
             -5,  0,  0,  0,  0,  0,  0, -5,
              0,  0,  0,  5,  5,  0,  0,  0,
        ];
        let square_table_dame = vec![
            -20, -10, -10,  -5,  -5, -10, -10, -20,
            -10,   0,   0,   0,   0,   0,   0, -10,
            -10,   0,   5,   5,   5,   5,   0, -10,
             -5,   0,   5,   5,   5,   5,   0,  -5,
              0,   0,   5,   5,   5,   5,   0,  -5,
            -10,   5,   5,   5,   5,   5,   0, -10,
            -10,   0,   5,   0,   0,   0,   0, -10,
            -20, -10, -10,  -5,  -5, -10, -10, -20,
        ];
        let square_table_konig = vec![
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -30, -40, -40, -50, -50, -40, -40, -30,
            -20, -30, -30, -40, -40, -30, -30, -20,
            -10, -20, -20, -20, -20, -20, -20, -10,
             20,  20,   0,   0,   0,   0,  20,  20,
             20,  30,  10,   0,   0,  10,  30,  20,
        ];

        Self {
            board,
            max_depth,
            piece_values,
            square_table: [
                vec![0; 64],
                square_table_bauer,
                square_table_springer,
                square_table_laeufer,
                square_table_turm,
                square_table_dame,
                square_table_konig,
            ],
            leaves_reached: 0,
        }
    }

    pub fn random_response(&self) -> Option<ChessMove> {
        let mg = MoveGen::new_legal(&self.board);
        mg.choose(&mut rand::thread_rng())
    }

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

    fn endgame_evaluation(&self, board: Board) -> i32 {
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

    pub fn alpha_beta(
        &mut self,
        board: Board,
        depth: u32,
        mut alpha: i32,
        mut beta: i32,
        maximiser: bool,
    ) -> (Option<ChessMove>, i32) {
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

    pub fn total_leaves(&mut self) -> u64 {
        let leaves = self.leaves_reached;
        self.leaves_reached = 0;
        leaves
    }
}

pub struct UCIEngine {
    engine: Arc<Mutex<Engine>>,
    is_running: bool,
    current_best_move: Option<ChessMove>,
}

impl UCIEngine {
    pub fn new() -> Self {
        let engine = Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        Self {
            engine: Arc::new(Mutex::new(engine)),
            is_running: false,
            current_best_move: None,
        }
    }

    pub fn uci(&self) {
        println!("id name Jomfish 2 - C");
        println!("id author Jimmy Luong");
        println!("uciok");
    }

    pub fn is_ready(&self) {
        println!("readyok");
    }

    pub fn position(&self, command: &str) {
        if command.starts_with("startpos") {
            {
                let mut engine = self.engine.lock().unwrap();
                engine.board = Board::default();
            }
            if let Some(idx) = command.find(" moves ") {
                let moves_str = &command[idx + 7..];
                self.apply_moves(moves_str);
            }
        } else if command.starts_with("fen ") {
            let parts: Vec<&str> = command.split(" moves ").collect();
            let fen_str = parts[0][4..].trim();
            if let Some(parsed_board) = Board::from_fen(fen_str.to_string()) {
                {
                    let mut engine = self.engine.lock().unwrap();
                    engine.board = parsed_board;
                }
            }
            if parts.len() > 1 {
                self.apply_moves(parts[1]);
            }
        } else {
            let parts: Vec<&str> = command.split(" moves ").collect();
            let fen_str = parts[0].trim();
            if let Some(parsed_board) = Board::from_fen(fen_str.to_string()) {
                {
                    let mut engine = self.engine.lock().unwrap();
                    engine.board = parsed_board;
                }
            }
            if parts.len() > 1 {
                self.apply_moves(parts[1]);
            }
        }
    }

    fn apply_moves(&self, moves_str: &str) {
        let moves: Vec<&str> = moves_str.split_whitespace().collect();
        let mut engine = self.engine.lock().unwrap();
        for mv_str in moves {
            if let Some(mv) = parse_uci_move(mv_str) {
                engine.board = engine.board.make_move_new(mv);
            }
        }
    }

    pub fn go(&mut self, command: &str) {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        let mut depth = 5;
        if let Some(idx) = tokens.iter().position(|&s| s == "depth") {
            if let Some(s) = tokens.get(idx + 1) {
                if let Ok(d) = s.parse::<u32>() {
                    depth = d;
                }
            }
        }
        let engine_arc = Arc::clone(&self.engine);
        let handle = thread::spawn(move || {
            let mut engine = engine_arc.lock().unwrap();
            engine.iterative_deepening(depth)
        });
        if let Ok(best_move) = handle.join() {
            self.current_best_move = best_move;
        }
        if let Some(mv) = self.current_best_move {
            println!("bestmove {}", mv);
        } else {
            println!("bestmove 0000");
        }
    }

    pub fn stop(&self) {
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn main_loop(&mut self) {
        self.is_running = true;
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        while self.is_running {
            print!("> ");
            stdout.flush().unwrap();
            let mut line = String::new();
            if stdin.lock().read_line(&mut line).is_err() {
                break;
            }
            let command = line.trim();
            if command.is_empty() {
                continue;
            }
            if command == "uci" {
                self.uci();
            } else if command == "isready" {
                self.is_ready();
            } else if command.starts_with("position") {
                self.position(&command[9..]);
            } else if command.starts_with("go") {
                let params = if command.len() > 2 { &command[2..].trim() } else { "" };
                self.go(params);
            } else if command == "stop" {
                self.stop();
            } else if command == "quit" {
                self.quit();
            }
        }
    }
}

fn is_valid_fen(fen: &str) -> bool {
    fen.split_whitespace().count() >= 6
}

fn main() {
    println!("id name Jomfish 2 - C");
    println!("id author Jimmy Luong");
    println!("uciok");
    let mut uci_engine = UCIEngine::new();
    uci_engine.main_loop();
}
