use crate::board::{A1, A8, BOARD_SIDE};

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
