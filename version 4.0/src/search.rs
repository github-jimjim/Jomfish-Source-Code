use log::info;
use std::cmp::max;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::board::{after_move, can_check, gen_moves, move_value, nullmove, BoardState};
use crate::pieces::Square;

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
