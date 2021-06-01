// 盤面を持っているモジュール
use crate::constant;

pub struct _Board {
    b: [[i32; constant::SIZE]; constant::SIZE],
    king: (i32, i32),
}

impl _Board {
    pub fn _init() -> _Board {
        _Board {
            b: [[0; constant::SIZE]; constant::SIZE],
            king: (4, 4),
        }
    }

    pub fn show_board(&self, k_show:bool) {
        let mut cnt:i32 = 0;
        for _row in &self.b {
            for _i in 0.._row.len() {
                if k_show && self.king.0 == _i as i32 && self.king.1 == cnt {
                    print!("|  K ");
                } else {
                    print!("| {0: >2} ", _row[_i]);
                }
            }
            println!("|");
            cnt += 1;
        }
    }

    pub fn set_board(&mut self, _x:usize, _y:usize, _value:i32) {
        self.b[_y][_x] = _value;
    }

    pub fn get_board_value(&self, _x:usize, _y:usize) -> i32 {
        return self.b[_y][_x];
    }

    pub fn move_king(&mut self, _x:i32, _y:i32) {
        self.king = (self.king.0+_x, self.king.1+_y);
    }

    pub fn get_king(&self) -> (i32, i32) {
        return self.king;
    }
}