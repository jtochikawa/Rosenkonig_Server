use crate::board;
use crate::player;
use crate::constant;
use crate::manager;
use big_s::S;
use rand::seq::SliceRandom;

pub struct _Game {
    _b: board::_Board,
    _deck: Vec<String>,
    _discard: Vec<String>,
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
            _discard: vec![],
        }
    }

    pub fn game_loop(&mut self) {
        let mut rng = rand::thread_rng();
        let v = [1, -1];
        self._deck.shuffle(&mut rng);
        let hands = self.deal();
        let mut players = [player::_Player::init(&hands[0]), player::_Player::init(&hands[1])];
        let mut c: usize = 0 as usize;
        'game: loop {
            self.show(&players);
            let mov = players[c].input_mov();
            match mov.trim() {
                "exit" => break 'game,
                "draw" => {
                    if manager::able_to_draw(&players[c]) {
                        self.update_hand(&mut players[c]);
                    } else {
                        continue 'game;
                    }
                },
                "pass" => {},
                _ =>  {
                    if manager::is_regal_move(&self._b, &mov) {
                        self.update_board(&mov, v[c]);
                        players[c].discard_card(&mov);
                    } else {
                        continue 'game;
                    }
                },
            }
            println!("{}", mov);
            c = 1 - c;
        }
    }

    // 上をNとして上のプレイヤーを後手
    pub fn show(&self, players:&[player::_Player; 2]) {
        println!("deck: {:?}", self._deck);
        println!("discard: {:?}", self._discard);
        print!("Player1: ");
        players[1].show_hand();
        self._b.show_board();
        print!("Player0: ");
        players[0].show_hand();
    }

    pub fn deal(&mut self) -> [Vec<String>; 2] {
        let mut hands: [Vec<String>; 2] = Default::default();
        for index in 0..10 {
            let dir = self._deck.pop().unwrap();
            hands[index%2].push(dir);
        }
        hands
    }

    pub fn update_board(&mut self, _mov:&String, _value:i32) {
        let &(x, y) = constant::CARDS.get(_mov).unwrap();
        self._b.move_king(x, y);
        let king = self._b.get_king();
        self._b.set_board(king.0 as usize, king.1 as usize, _value);
        self._discard.push(_mov.to_string());
    }

    pub fn update_hand(&mut self, _player:&mut player::_Player) {
        let card = self._deck.pop().unwrap();
        _player.add_card(card);
    }
}