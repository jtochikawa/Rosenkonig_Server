use crate::board;

pub struct _Game {
    _b: board::_Board,
}

impl _Game {
    pub fn _init () -> _Game {
        _Game {
            _b: board::_Board::_init(),
        }
    }

    pub fn game_loop(&mut self) {
        self._b.set_board(4, 4);
        self._b.show_board();
    }
}