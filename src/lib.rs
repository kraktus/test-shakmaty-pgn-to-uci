use std::{fmt::Write, ops::ControlFlow};

use arrayvec::{ArrayString, ArrayVec};
use pgn_reader::{SanPlus, Skip, Visitor};
use shakmaty::{CastlingMode, Chess, Position, san::San};

type SanList = ArrayVec<San, 512>;

// TODO check if max size is enough
type UciString = ArrayString<2048>;

// 1. e4 e6 2. d4 d5 3. e5 c5 4. Nf3 Qb6 5. Bd3 Bd7 6. O-O Bb5 7. Re1 Bxd3 8. Qxd3 cxd4 9. a3 Nc6 10. b4 a6 11. Bb2 Rc8 12. Nbd2 Qc7 13. Nb3 Nge7 14. Nbxd4 Nxd4 15. Bxd4 Qxc2 16. Qe3 Nf5 17. Qf4 Nxd4 18. Qxd4 Be7 19. Qb6 O-O 20. Qxb7 Qc6 21. Qxc6 Rxc6 22. Rac1 Rfc8 23. Rxc6 Rxc6 24. Ra1 h6 25. Kf1 Kf8 26. Ke2 Rc2+ 27. Ke3 Ke8 28. Nd4 Rc3+ 29. Kd2 Rc4 30. Kd3 Kd7 31. Ra2 Kc7 32. Rc2 Rxc2 33. Nxc2 Kb6 34. Kd4 Kb5 35. Kc3 Bg5 36. g3 Bc1 37. Kb3 f6 38. exf6 gxf6 39. Nd4+ Kb6 40. Nxe6 Bd2 41. Ng7 Be1 42. f3 d4 43. Kc4 Bf2 44. Nf5 h5 45. Nxd4 Bxd4 46. Kxd4 Kb5 47. g4 h4 48. f4 Ka4 49. Kc4 Kxa3 50. g5
// do not use rust-pgn-parser, assume no variations /!\
// implementing own parser never end well, but...
fn str_to_san(x: &str) -> SanList {
    let mut san_list = SanList::new();
    for (i, san_ascii) in x.split(' ').enumerate() {
        if i % 3 != 0 {
            san_list.push(San::from_ascii(san_ascii.as_bytes()).expect("Invalid SAN"))
        }
    }
    san_list
}

pub fn str_to_uci(pgn: &str) -> UciString {
    let l_san = str_to_san(pgn);
    let san_length = l_san.len();
    let mut pos = Chess::new();
    let mut uci_string = UciString::new();
    for (i, san) in l_san.iter().enumerate() {
        let m = san.to_move(&pos).expect("Illegal move");
        let uci = m.to_uci(CastlingMode::Chess960);
        write!(uci_string, "{uci}").expect("writing uci failed");
        if i != san_length - 1 {
            uci_string.push(' ');
        }

        pos.play_unchecked(m);
    }
    uci_string
}

pub const EXAMPLE_PGN: &str = "1. e4 e6 2. d4 d5 3. e5 c5 4. Nf3 Qb6 5. Bd3 Bd7 6. O-O Bb5 7. Re1 Bxd3 8. Qxd3 cxd4 9. a3 Nc6 10. b4 a6 11. Bb2 Rc8 12. Nbd2 Qc7 13. Nb3 Nge7 14. Nbxd4 Nxd4 15. Bxd4 Qxc2 16. Qe3 Nf5 17. Qf4 Nxd4 18. Qxd4 Be7 19. Qb6 O-O 20. Qxb7 Qc6 21. Qxc6 Rxc6 22. Rac1 Rfc8 23. Rxc6 Rxc6 24. Ra1 h6 25. Kf1 Kf8 26. Ke2 Rc2+ 27. Ke3 Ke8 28. Nd4 Rc3+ 29. Kd2 Rc4 30. Kd3 Kd7 31. Ra2 Kc7 32. Rc2 Rxc2 33. Nxc2 Kb6 34. Kd4 Kb5 35. Kc3 Bg5 36. g3 Bc1 37. Kb3 f6 38. exf6 gxf6 39. Nd4+ Kb6 40. Nxe6 Bd2 41. Ng7 Be1 42. f3 d4 43. Kc4 Bf2 44. Nf5 h5 45. Nxd4 Bxd4 46. Kxd4 Kb5 47. g4 h4 48. f4 Ka4 49. Kc4 Kxa3 50. g5";

pub fn str_to_uci_visitor(pgn: &str) -> UciString {
    let mut visitor = UciVisitor::default();
    let mut reader = pgn_reader::Reader::new(pgn.as_bytes());
    reader.read_game(&mut visitor).expect("Reading PGN failed");
    visitor.uci
}

struct UciVisitor {
    pos: Chess,
    is_first_move: bool,
    pub uci: UciString,
}

impl Default for UciVisitor {
    fn default() -> Self {
        Self {
            pos: Chess::new(),
            uci: UciString::new(),
            is_first_move: true,
        }
    }
}

impl Visitor for UciVisitor {
    type Tags = ();
    type Movetext = ();
    type Output = ();

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        ControlFlow::Continue(())
    }
    fn san(&mut self, _: &mut Self::Movetext, san_plus: SanPlus) -> ControlFlow<Self::Output> {
        let m = san_plus.san.to_move(&self.pos).expect("Illegal move");
        let uci = m.to_uci(CastlingMode::Chess960);
        if self.is_first_move {
            self.is_first_move = false;
        } else {
            self.uci.push(' ');
        }
        write!(self.uci, "{uci}").expect("writing uci failed");
        self.pos.play_unchecked(m);
        ControlFlow::Continue(())
    }

    fn begin_variation(&mut self, _: &mut Self::Movetext) -> ControlFlow<Self::Output, Skip> {
        ControlFlow::Continue(Skip(true))
    }

    fn begin_movetext(&mut self, _: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(())
    }
    fn end_game(&mut self, _: Self::Movetext) -> Self::Output {
        ()
    }
}
