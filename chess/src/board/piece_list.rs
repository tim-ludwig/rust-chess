use crate::board::position::Position;
use crate::piece::{Color, Piece, PieceType};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct PieceList {
    lists: HashMap<Color, HashMap<PieceType, HashSet<Position>>>,
}

impl PieceList {
    pub fn new() -> Self {
        let mut l = Self {
            lists: HashMap::new(),
        };

        for c in Color::iter() {
            l.lists.insert(c, HashMap::new());
            let cl = l.lists.get_mut(&c).unwrap();

            for t in PieceType::iter() {
                cl.insert(t, HashSet::new());
            }
        }

        l
    }

    fn get(&self, p: &Piece) -> &HashSet<Position> {
        self.lists
            .get(&p.color)
            .unwrap()
            .get(&p.piece_type)
            .unwrap()
    }

    fn get_mut(&mut self, p: &Piece) -> &mut HashSet<Position> {
        self.lists
            .get_mut(&p.color)
            .unwrap()
            .get_mut(&p.piece_type)
            .unwrap()
    }

    pub fn put_piece(&mut self, p: &Piece, pos: &Position) {
        self.get_mut(p).insert(*pos);
    }

    pub fn remove_piece(&mut self, p: &Piece, pos: &Position) {
        self.get_mut(p).remove(pos);
    }

    pub fn pos_of_piece(&self, p: &Piece) -> impl Iterator<Item = Position> + '_ {
        self.get(p).iter().copied()
    }

    pub fn move_piece(&mut self, p: &Piece, from: &Position, to: &Position) {
        let l = self.get_mut(p);
        l.remove(from);
        l.insert(*to);
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece_list::PieceList;
    use crate::board::position::Position;
    use crate::piece::Piece;
    use std::str::FromStr;

    #[test]
    fn put_piece() {
        let mut l = PieceList::new();

        let wr = Piece::from_fen_char(&'R').unwrap();
        let wp = Piece::from_fen_char(&'P').unwrap();

        let br = Piece::from_fen_char(&'r').unwrap();
        let bp = Piece::from_fen_char(&'p').unwrap();

        let a1 = "a1".parse().unwrap();
        let from = "a8".parse().unwrap();
        let to = "g8".parse().unwrap();
        let h8 = "h8".parse().unwrap();

        l.put_piece(&wr, &a1);
        l.put_piece(&wp, &"a2".parse().unwrap());
        l.put_piece(&br, &from);
        l.put_piece(&br, &h8);
        l.put_piece(&bp, &"a7".parse().unwrap());

        assert!(l.pos_of_piece(&wr).find(|p| p == &a1).is_some());
        assert!(l.pos_of_piece(&br).find(|p| p == &from).is_some());
        assert!(l.pos_of_piece(&br).find(|p| p == &h8).is_some());

        l.move_piece(&br, &from, &to);

        assert!(l.pos_of_piece(&wr).find(|p| p == &a1).is_some());
        assert!(l.pos_of_piece(&br).find(|p| p == &from).is_none());
        assert!(l.pos_of_piece(&br).find(|p| p == &to).is_some());
        assert!(l.pos_of_piece(&br).find(|p| p == &h8).is_some());

        l.remove_piece(&br, &to);

        assert!(l.pos_of_piece(&wr).find(|p| p == &a1).is_some());
        assert!(l.pos_of_piece(&br).find(|p| p == &from).is_none());
        assert!(l.pos_of_piece(&br).find(|p| p == &to).is_none());
        assert!(l.pos_of_piece(&br).find(|p| p == &h8).is_some());
    }
}
