// ゲームを回すモジュール

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
    _piece_num: u32,
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
            _piece_num: 0,
        }
    }

    pub fn game_loop(&mut self) {
        let mut rng = rand::thread_rng();
        self._deck.shuffle(&mut rng);
        let hands = self.deal();
        let mut players = [player::_Player::init(&hands[0], 1), player::_Player::init(&hands[1], -1)];
        let mut c: usize = 0 as usize;
        'game: loop {
            if manager::is_end(&players, self._piece_num) { break 'game; }
            self.show(&players);
            let movable_list = manager::create_movable_list(&self._b, &players[c]);
            if movable_list.len() < 1 { 
                c = 1 - c;
                continue 'game; 
            }
            let mov = players[c].input_mov();
            match mov.trim() {
                "exit" => break 'game,
                "draw" => {
                    if movable_list.contains(&mov) {
                        self.update_hand(&mut players[c]);
                    } else {
                        continue 'game;
                    }
                },
                _ =>  {
                    if movable_list.contains(&mov) {
                        self.update_board(&mov, &mut players[c]);
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
        println!("piece num: {0}", self._piece_num);
        print!("Player1: ");
        players[1].show_hand();
        println!("Kight: {0}", players[1].get_kight_num());
        self._b.show_board();
        print!("Player0: ");
        players[0].show_hand();
        println!("Kight: {0}", players[0].get_kight_num());
    }

    pub fn deal(&mut self) -> [Vec<String>; 2] {
        let mut hands: [Vec<String>; 2] = Default::default();
        for index in 0..10 {
            let dir = self._deck.pop().unwrap();
            hands[index%2].push(dir);
        }
        hands
    }

    pub fn update_board(&mut self, _mov:&String, _player:&mut player::_Player) {
        let &(x, y) = constant::CARDS.get(_mov).unwrap();
        self._b.move_king(x, y);
        let king = self._b.get_king();
        if manager::is_empty(&self._b, king.0 as usize, king.1 as usize) {
            self._piece_num += 1;
        } else {
            _player.dic_kight();
        }
        self._b.set_board(king.0 as usize, king.1 as usize, _player.get_value());
        self._discard.push(_mov.to_string());
    }

    pub fn update_hand(&mut self, _player:&mut player::_Player) {
        let card = self._deck.pop().unwrap();
        _player.add_card(card);
    }
}