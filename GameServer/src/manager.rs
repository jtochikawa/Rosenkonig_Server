/* 
    ゲーム、ボードに対して処理を行って良いか管理するモジュール
*/

use crate::player;
use crate::board;
use crate::constant;

pub fn able_to_draw(p:&player::_Player) -> bool {
    return p.get_hand_length() < 5;
}

pub fn create_movable_list(b:&board::_Board, p:&player::_Player) -> Vec<String> {
    let mut movable_list : Vec<String> = vec![];
    if able_to_draw(&p) {
        movable_list.push("draw".to_string());
    }
    for index in 0..p.get_hand_length() {
        let m = p.get_card(index);
        if is_regal_move(&b, &m, &p) {
            movable_list.push(m);
        }
    }

    return movable_list;
}

pub fn is_regal_move(b:&board::_Board, m:&String, _player:&player::_Player) -> bool {
    let &(x, y) = constant::CARDS.get(m).unwrap();
    let king = b.get_king();
    let new_pos = (king.0+x, king.1+y);
    if is_range(new_pos.0, new_pos.1) {
        if is_opp(b, new_pos.0 as usize, new_pos.1 as usize, _player.get_value()) { return _player.get_kight_num() > 0; }
        else if is_empty(b, new_pos.0 as usize, new_pos.1 as usize) { return true; }
    }
    return false;
}

pub fn is_range(x:i32, y:i32) -> bool {
    return x > -1 && x < *constant::SIZE && y > -1 && y < *constant::SIZE;
}

pub fn is_opp(b:&board::_Board, x:usize, y:usize, v:i32) -> bool {
    return b.get_board_value(x, y) == -v;
}

pub fn is_empty(b:&board::_Board, x:usize, y:usize) -> bool {
    return b.get_board_value(x, y) == *constant::EMPTY;
}

pub fn is_end(players:&[player::_Player; 2], piece_num: u32) -> bool {
    return (players[0].get_pass_flag() && players[1].get_pass_flag()) || (piece_num == *constant::PIECE_NUM);
}