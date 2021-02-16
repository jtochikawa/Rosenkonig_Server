pub struct _Player {
    hand: Vec<String>,
}

impl _Player {
    pub fn init(h:&Vec<String>) -> _Player {
        _Player{
            hand: h.to_vec(),
        }
    }

    pub fn show_hand(&self) {
        for _len in 0..self.hand.len() {
            print!("{0}:{1} ", _len, self.hand[_len]);
        }
        println!();
    }
}