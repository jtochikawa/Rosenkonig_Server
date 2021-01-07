pub struct _Board {
    b: [[i32; 9]; 9],
    king: (i32, i32),
}

impl _Board {
    pub fn _init() -> _Board {
        _Board {
            b: [[0; 9]; 9],
            king: (5, 5),
        }
    }

    pub fn show_board(&self) {
        for _row in &self.b {
            for _i in 0.._row.len() {
                print!("| {} ", _row[_i]);
            }
            println!("|");
        }
    }

    pub fn set_board(&mut self, _x:usize, _y:usize) {
        self.b[_y][_x] = 1;
    }

    pub fn move_king(&mut self, _x:i32, _y:i32) {
        self.king = (_x, _y);
    }
}