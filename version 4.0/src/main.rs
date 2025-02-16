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
