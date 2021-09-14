pub mod constants;
pub mod pieces;
use crate::mailbox::constants::*;
use crate::mailbox::pieces::*;
use std::fmt::Display;

pub struct MailboxBoard {
    array: [Square; 128],
}

#[derive(PartialEq)]
pub struct Move {
    from: RankFile,
    to: RankFile,
}

use std::fmt;

impl Debug for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(
            fmt,
            "0x{:02x} -> 0x{:02x}, {} -> {}",
            self.from,
            self.to,
            display_RankFile(self.from),
            display_RankFile(self.to)
        )
    }
}

fn display_RankFile(rank_file: RankFile) -> &'static str {
    match rank_file {
        0x00 => "A1",
        0x01 => "A2",
        0x02 => "A3",
        0x03 => "A4",
        0x04 => "A5",
        0x05 => "A6",
        0x06 => "A7",
        0x07 => "A8",
        0x10 => "B1",
        0x11 => "B2",
        0x12 => "B3",
        0x13 => "B4",
        0x14 => "B5",
        0x15 => "B6",
        0x16 => "B7",
        0x17 => "B8",
        0x20 => "C1",
        0x21 => "C2",
        0x22 => "C3",
        0x23 => "C4",
        0x24 => "C5",
        0x25 => "C6",
        0x26 => "C7",
        0x27 => "C8",
        0x30 => "D1",
        0x31 => "D2",
        0x32 => "D3",
        0x33 => "D4",
        0x34 => "D5",
        0x35 => "D6",
        0x36 => "D7",
        0x37 => "D8",
        0x40 => "E1",
        0x41 => "E2",
        0x42 => "E3",
        0x43 => "E4",
        0x44 => "E5",
        0x45 => "E6",
        0x46 => "E7",
        0x47 => "E8",
        0x50 => "F1",
        0x51 => "F2",
        0x52 => "F3",
        0x53 => "F4",
        0x54 => "F5",
        0x55 => "F6",
        0x56 => "F7",
        0x57 => "F8",
        0x60 => "G1",
        0x61 => "G2",
        0x62 => "G3",
        0x63 => "G4",
        0x64 => "G5",
        0x65 => "G6",
        0x66 => "G7",
        0x67 => "G8",
        0x70 => "H1",
        0x71 => "H2",
        0x72 => "H3",
        0x73 => "H4",
        0x74 => "H5",
        0x75 => "H6",
        0x76 => "H7",
        0x77 => "H8",
        _ => "Invalid",
    }
}

pub trait Movable {
    fn moves(&self, rank_file: RankFile) -> Vec<Move>;
}

impl Movable for Piece {
    fn moves(&self, rank_file: RankFile) -> Vec<Move> {
        match self {
            Piece::White(p) => p.moves(rank_file),
            Piece::Black(p) => p.moves(rank_file),
        }
    }
}

use self::from_rank_file as ffr;
use self::rank_file as fr;
use core::fmt::Debug;

impl Movable for PieceType {
    fn moves(&self, rank_file: RankFile) -> Vec<Move> {
        use std::env::set_var;
        set_var("RUST_BACKTRACE", "full");
        match self {
            PieceType::Knight => {
                // (+2, +1), (+2, -1), (-2, +1), ()
                //
                let (file, rank) = fr(rank_file as u8);
                let mut moves = vec![];
                // Movement vectors: (delta file, delta rank),
                for (df, dr) in [
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                    (-1, 2),
                    (-1, -2),
                    (1, 2),
                    (1, -2),
                ] {
                    let file = file as i8;
                    let rank = rank as i8;
                    let to = ffr(file.wrapping_add(df) as u8, rank.wrapping_add(dr) as u8);
                    moves.push(Move {
                        from: rank_file,
                        to,
                    })
                }
                moves
            }
            _ => todo!(),
        }
    }
}

impl Default for MailboxBoard {
    fn default() -> Self {
        let mut array: [Square; 128] = [Default::default(); 128];
        for i in (0..array.len()).filter(|x| x & 0x88 != 0) {
            array[i] = Square::Invalid;
        }
        Self { array }
    }
}

const STARTING_PIECES: [(RankFile, Piece); 16] = [
    (A1, Piece::White(PieceType::Rook)),
    (A2, Piece::White(PieceType::Knight)),
    (A3, Piece::White(PieceType::Bishop)),
    (A4, Piece::White(PieceType::Queen)),
    (A5, Piece::White(PieceType::King)),
    (A6, Piece::White(PieceType::Bishop)),
    (A7, Piece::White(PieceType::Knight)),
    (A8, Piece::White(PieceType::Rook)),
    (H1, Piece::White(PieceType::Rook)),
    (H2, Piece::White(PieceType::Knight)),
    (H3, Piece::White(PieceType::Bishop)),
    (H4, Piece::White(PieceType::Queen)),
    (H5, Piece::White(PieceType::King)),
    (H6, Piece::White(PieceType::Bishop)),
    (H7, Piece::White(PieceType::Knight)),
    (H8, Piece::White(PieceType::Rook)),
];
const WHITE_PAWN_RANK: u8 = 0x1;
const BLACK_PAWN_RANK: u8 = 0x6;

pub struct DisplayBoard {
    chars: [&'static str; 128],
}

impl Display for DisplayBoard {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for rank in (0x0..0x8).rev() {
            write!(fmt, "{} ", rank + 1);
            for file in (0x0..0x8).rev() {
                // The swap in rank / file is intentional.
                write!(fmt, "{} ", self.chars[from_rank_file(rank, file)])?;
            }
            writeln!(fmt);
        }
        writeln!(fmt, "  A B C D E F G H");
        Ok(())
    }
}

impl MailboxBoard {
    /// Initialize game state with starting position.
    pub fn new() -> Self {
        let mut mailbox = Self::default();
        for file in 0x0..0x8 {
            mailbox.set(
                Square::Occupied(Piece::White(PieceType::Pawn)),
                from_rank_file(WHITE_PAWN_RANK, file),
            );
            mailbox.set(
                Square::Occupied(Piece::Black(PieceType::Pawn)),
                from_rank_file(BLACK_PAWN_RANK, file),
            );
        }
        for (pos, piece) in STARTING_PIECES {
            mailbox.set(Square::Occupied(piece), pos);
        }
        mailbox
    }
    fn get_pieces(&self) -> [&'static str; 128] {
        let mut chars = [""; 128];
        for (i, square) in self.array.iter().enumerate() {
            chars[i] = square.char();
        }
        chars
    }
    pub fn display(&self) -> DisplayBoard {
        DisplayBoard {
            chars: self.get_pieces(),
        }
    }
}

type File = u8;
type Rank = u8;
type RankFile = usize;

/// 0x88 conversion to array index.
/// rank_file(0x42) for example should return E2 (4, 2).
pub const fn rank_file(sq0x88: u8) -> (Rank, File) {
    (sq0x88 >> 4, sq0x88 & 7)
}

pub const fn from_rank_file(file: File, rank: Rank) -> RankFile {
    (16u8.wrapping_mul(rank)).wrapping_add(file) as RankFile
}

impl MailboxBoard {
    pub fn index(&self, rank_file: RankFile) -> Square {
        self.array[rank_file]
    }
    pub fn set(&mut self, value: Square, rank_file: RankFile) {
        self.array[rank_file] = value;
    }
}

pub const KNIGHTS_TOUR: &'static str = "g7 h5 f6 e4 g3 h1 f2 d1 b2 a4 c3 d5 b6 a8 c7 b5 a7 c8 d6 c4 a3 b1 d2 f1 h2 g4 e3 f5 h6 g8 e7 c6 d8 b7 a5 b3 a1 c2 d4 f3 e1 g2 h4 g6 h8 f7 g5 h7 f8 e6 f4 h3 g1 e2 c1 a2 b4 d3 c5 a6 b8 d7 e5";
pub fn read_positions(s: &str) -> Vec<RankFile> {
    s.split_ascii_whitespace()
        .map(|s| {
            let mut chars = s.chars();
            let file = chars.next().unwrap() as u8 - 'a' as u8;
            let rank = chars.next().unwrap() as u8 - '0' as u8 - 1;
            from_rank_file(file, rank)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::mailbox;
    use crate::mailbox::constants::*;
    use crate::mailbox::*;

    #[test]
    fn _rank_file() {
        let mut mb = MailboxBoard::default();
        mb.set(Square::Invalid, C2);
        assert_eq!(mb.index(C2), Square::Invalid);
        assert_eq!(mailbox::from_rank_file(C, 2), 0x22);
    }

    #[test]
    fn rank_files() {
        let (file, rank) = rank_file(E6 as u8);
        assert_eq!(
            E6,
            from_rank_file(file, rank),
            "{} -- {}",
            display_RankFile(E6),
            display_RankFile(from_rank_file(file, rank))
        );
        assert_eq!(
            E4,
            ffr(file, (rank - 2) as u8),
            "{} -- {}\n{}",
            display_RankFile(E4),
            display_RankFile(from_rank_file(file, rank + 0)),
            rank
        );
    }
    #[test]
    fn valids() {
        let mb = MailboxBoard::default();
        (0..128).filter(|x| x & 0x88 == 0).for_each(|sq| {
            assert!(
                mb.array[sq] != Square::Invalid,
                "{}, {:?}",
                sq,
                rank_file(sq as u8)
            )
        });
    }
    #[test]
    fn invalids() {
        let mb = MailboxBoard::default();
        assert!((0..128)
            .filter(|x| x & 0x88 != 0)
            .all(|sq| mb.array[sq] == Square::Invalid));
    }

    #[test]
    fn endianness() {
        let square = E8;
        let (file, rank) = rank_file(square as u8);
        let fr = from_rank_file(4, 7);
        assert_eq!(rank_file(E8 as u8), (4, 7));
        assert_eq!(fr, square);
        assert_eq!(file, 0x4);
        assert_eq!(rank, 0x7);
    }

    #[test]
    fn simple_knight_sanity() {
        let moves = PieceType::Knight.moves(E8);
        assert!(moves.contains(&Move { from: E8, to: F6 }));
        assert!(moves.contains(&Move { from: E8, to: D6 }));
        assert!(moves.contains(&Move { from: E8, to: G7 }));
        assert!(moves.contains(&Move { from: E8, to: C7 }));
    }

    #[test]
    fn knights_tour() {
        // e8 is starting position -> these moves should be possible
        let mut current_sq = E8;
        assert_eq!('e' as u8 - 'a' as u8, 0x4);
        assert_eq!('8' as u8 - '0' as u8 - 1, 0x7);
        let mut visited = 0;
        for (i, rank_file) in read_positions(KNIGHTS_TOUR).iter().enumerate() {
            let moves = PieceType::Knight.moves(current_sq);
            if moves.iter().any(|m| m.to == *rank_file) {
                visited |= rank_file;
                current_sq = *rank_file;
            } else {
                panic!(
                    "[{}] Knight didn't believe he could go to {:?} from {:?}\n0x{:02x} -> 0x{:02x}\n{:?}",
                    i,
                    display_RankFile(*rank_file),
                    display_RankFile(current_sq),
                    current_sq,
                    rank_file,
                    moves
                );
            }
        }
        assert_eq!(visited, 0b01110111, "{:08b} vs {:08b}", visited, 0b01110111);
    }
}
