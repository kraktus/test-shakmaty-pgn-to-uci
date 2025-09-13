use test_shakmaty_pgn_to_uci::{EXAMPLE_PGN, str_to_uci, str_to_uci_visitor};

fn main() {
    let uci = str_to_uci(EXAMPLE_PGN);
    println!("UCI: {}", uci);
    let visitor_uci = str_to_uci_visitor(EXAMPLE_PGN);
    println!("UCI visitor: {}", visitor_uci);
    assert_eq!(uci, visitor_uci);
}
