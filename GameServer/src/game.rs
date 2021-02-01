use crate::board;
use big_s::S;
use maplit::hashmap;
use std::collections::HashMap;

pub struct _Game {
    _b: board::_Board,
}

const DECK: HashMap<String, (String, i32)> = hashmap![
    S("E1") => (S("East"), 1),
    S("E2") => (S("East"), 2),
    S("E3") => (S("East"), 3),
    S("W1") => (S("West"), 1),
    S("W2") => (S("West"), 2),
    S("W3") => (S("West"), 3),
    S("N1") => (S("North"), 1),
    S("N2") => (S("North"), 2),
    S("N3") => (S("North"), 3),
    S("S1") => (S("South"), 1),
    S("S2") => (S("South"), 2),
    S("S3") => (S("South"), 3),
];

impl _Game {
    pub fn _init () -> _Game {
        _Game {
            _b: board::_Board::_init(),
        }
    }

    pub fn game_loop(&mut self) {
        self._b.set_board(4, 4, 1);
        self._b.show_board();
        println!();
        self._b.move_king(5,5);
        self._b.show_board();
    }
}