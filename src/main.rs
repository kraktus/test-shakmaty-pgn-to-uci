use test_shakmaty_pgn_to_uci::{
    str_to_uci, EXAMPLE_PGN
};



fn main() {
    let uci = str_to_uci(EXAMPLE_PGN);
    println!("UCI: {}", uci);
}
