use crate::board;
use big_s::S;
use std::collections::HashMap;
use maplit::hashmap;

pub struct _Game {
    _b: board::_Board,
}

lazy_static! {
    static ref CARDS: HashMap<String, (i32, i32)> = {
        let m = hashmap! {
            S("E1") => (1, 0),
            S("E2") => (2, 0),
            S("E3") => (3, 0),
            S("W1") => (-1, 0),
            S("W2") => (-2, 0),
            S("W3") => (-3, 0),
            S("N1") => (0, -1),
            S("N2") => (0, -2),
            S("N3") => (0, -3),
            S("S1") => (0, 1),
            S("S2") => (0, 2),
            S("S3") => (0, 3),
            S("NE1") => (1, -1),
            S("NE2") => (2, -2),
            S("NE3") => (3, -3),
            S("NW1") => (-1, -1),
            S("NW2") => (-2, -2),
            S("NW3") => (-3, -3),
            S("SE1") => (1, -1),
            S("SE2") => (2, -2),
            S("SE3") => (3, -3),
            S("SW1") => (-1, 1),
            S("SW2") => (-2, 2),
            S("SW3") => (-3, 3),
        };
        m
    };
}

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