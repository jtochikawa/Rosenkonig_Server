use crate::input;

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

    pub fn add_card(&mut self, _card:String) {
        self.hand.push(_card);
    }

    pub fn input_mov(&mut self) -> String {
        loop {
            let mov: String = input::read();
            if mov.trim() == "draw" {
                return mov;
            } else if self.hand.contains(&mov) {
                return mov;
            } else {
                return "exit".to_string();
            }
        }
    }

    pub fn get_hand_length(&self) -> usize {
        return self.hand.len();
    }

    pub fn discard_card(&mut self, mov:&String) {
        self.hand.retain(|x| x != mov);
    }
}