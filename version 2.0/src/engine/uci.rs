use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use chess::{Board, ChessMove};
use crate::engine::engine::Engine;
use crate::engine::utils::parse_uci_move;

pub struct UCIEngine {
    pub engine: Arc<Mutex<Engine>>,
    pub is_running: bool,
    pub current_best_move: Option<ChessMove>,
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
        println!("id name Jomfish 2");
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
            match parse_uci_move(mv_str) {
                Some(mv) => engine.board = engine.board.make_move_new(mv),
                None => println!("info string Invalid move: {}", mv_str),
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

        let best_move = thread::spawn(move || {
            let mut engine = engine_arc.lock().unwrap();
            engine.iterative_deepening(depth)
        })
        .join()
        .unwrap_or(None);

        self.current_best_move = best_move;

        if let Some(mv) = self.current_best_move {
            println!("bestmove {}", mv);
        } else {
            println!("bestmove 0000");
        }
    }

    pub fn stop(&self) {
        println!("info string Search stopped");
    }

    pub fn quit(&mut self) {
        self.is_running = false;
        println!("info string Engine quitting...");
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

            match command {
                "uci" => self.uci(),
                "isready" => self.is_ready(),
                "quit" => self.quit(),
                "stop" => self.stop(),
                _ if command.starts_with("position") => self.position(&command[9..]),
                _ if command.starts_with("go") => self.go(command),
                _ => println!("info string Unknown command: {}", command),
            }
        }
    }
}
