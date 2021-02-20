use crate::board;
use crate::player;
use big_s::S;
use std::collections::HashMap;
use maplit::hashmap;
use rand::seq::SliceRandom;

pub struct _Game {
    _b: board::_Board,
    _deck: Vec<String>,
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
            _deck: vec![
                S("E1"), S("E2"), S("E3"), S("W1"), S("W2"), S("W3"),
                S("N1"), S("N2"), S("N3"), S("S1"), S("S2"), S("S3"),
                S("NE1"), S("NE2"), S("NE3"), S("NW1"), S("NW2"), S("NW3"),
                S("SE1"), S("SE2"), S("SE3"), S("SW1"), S("SW2"), S("SW3"),
            ],
        }
    }

    pub fn game_loop(&mut self) {
        let mut rng = rand::thread_rng();
        let v = [1, -1];
        self._deck.shuffle(&mut rng);
        let hands = self.deal();
        let mut players = [player::_Player::init(&hands[0]), player::_Player::init(&hands[1])];
        println!("{:?}", self._deck);
        let mut cnt: u32 = 0;
        loop {
            self.show(&players);
            let c: usize = (cnt%2) as usize;
            let mov = players[c].input_mov();
            if mov.trim() == "exit" {
                break;
            }
            let &(x, y) = CARDS.get(&mov).unwrap();
            self._b.move_king(-v[c]*x, -v[c]*y);
            let king = self._b.get_king();
            self._b.set_board(king.0 as usize, king.1 as usize, v[c]);
            println!("{}", mov);
            cnt += 1;
        }
    }

    pub fn show(&self, players:&[player::_Player; 2]) {
        players[0].show_hand();
        self._b.show_board();
        players[1].show_hand();
    }

    pub fn deal(&mut self) -> [Vec<String>; 2] {
        let mut hands: [Vec<String>; 2] = Default::default();
        for index in 0..10 {
            let dir = self._deck.pop().unwrap();
            hands[index%2].push(dir);
        }
        hands
    }
}