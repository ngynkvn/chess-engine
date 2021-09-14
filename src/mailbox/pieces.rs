#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Pawn,
    Knight,
    Bishop,
    Rook,
}

impl PieceType {
    pub fn black_char(&self) -> &'static str {
        match self {
            PieceType::King => "♚",
            PieceType::Queen => "♛",
            PieceType::Rook => "♜",
            PieceType::Bishop => "♝",
            PieceType::Knight => "♞",
            PieceType::Pawn => "♟",
        }
    }
    pub fn white_char(&self) -> &'static str {
        match self {
            PieceType::King => "♔",
            PieceType::Queen => "♕",
            PieceType::Rook => "♖",
            PieceType::Bishop => "♗",
            PieceType::Knight => "♘",
            PieceType::Pawn => "♙",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    White(PieceType),
    Black(PieceType),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Square {
    Empty,
    Occupied(Piece),
    Invalid,
}

impl Piece {
    pub fn char(&self) -> &'static str {
        match self {
            Piece::White(piece_type) => piece_type.white_char(),
            Piece::Black(piece_type) => piece_type.black_char(),
        }
    }
}

impl Square {
    pub fn char(&self) -> &'static str {
        match self {
            Square::Empty | Square::Invalid => ".",
            Square::Occupied(piece) => piece.char(),
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::Empty
    }
}
