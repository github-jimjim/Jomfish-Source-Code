mod engine;

use engine::uci::UCIEngine;

fn main() {
    println!("id name Jomfish 2");
    println!("id author Jimmy Luong");
    println!("uciok");

    let mut uci_engine = UCIEngine::new();
    uci_engine.main_loop();
}
