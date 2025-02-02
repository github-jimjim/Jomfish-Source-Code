use chess::{ChessMove, File, Rank, Square};

pub fn piece_from_int(piece_type: u8) -> chess::Piece {
    match piece_type {
        1 => chess::Piece::Pawn,
        2 => chess::Piece::Knight,
        3 => chess::Piece::Bishop,
        4 => chess::Piece::Rook,
        5 => chess::Piece::Queen,
        6 => chess::Piece::King,
        _ => panic!("Ungültiger Piece-Typ: {}", piece_type),
    }
}

pub fn parse_uci_move(uci: &str) -> Option<ChessMove> {
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
            'q' => Some(chess::Piece::Queen),
            'r' => Some(chess::Piece::Rook),
            'b' => Some(chess::Piece::Bishop),
            'n' => Some(chess::Piece::Knight),
            _ => None,
        }
    } else {
        None
    };
    Some(ChessMove::new(source, dest, promotion))
}

pub fn is_valid_fen(fen: &str) -> bool {
    fen.split_whitespace().count() >= 6
}
