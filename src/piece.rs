enum Color {
    WHITE,
    BLACK
}

enum Piece {
    NONE,
    KING(Color),
    QUEEN(Color),
    ROOK(Color),
    BISHOP(Color),
    KNIGHT(Color),
    PAWN(Color)
}