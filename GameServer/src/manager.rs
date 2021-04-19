use crate::player;
use crate::board;
use crate::constant;

pub fn able_to_draw(p:&player::_Player) -> bool {
    return p.get_hand_length() < 5;
}

pub fn is_regal_move(b:&board::_Board, m:&String) -> bool {
    let &(x, y) = constant::CARDS.get(m).unwrap();
    let king = b.get_king();
    let new_pos = (king.0+x, king.1+y);
    return is_range(new_pos.0, new_pos.1) && is_empty(b, new_pos.0 as usize, new_pos.1 as usize);
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