// プレイヤーの基本のモジュール

use crate::input;
use crate::constant;

pub struct _Player {
    hand: Vec<String>,
    pass_flag: bool,
    value: i32,
    kight: i32,
}

impl _Player {
    pub fn init(h:&Vec<String>, v:i32) -> _Player {
        _Player{
            hand: h.to_vec(),
            pass_flag: false,
            value: v,
            kight: constant::KIGHT,
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

    pub fn get_pass_flag(&self) -> bool {
        return self.pass_flag;
    }

    pub fn set_pass_flag(&mut self, flag:bool) {
        self.pass_flag = flag;
    }

    pub fn get_value(&self) -> i32 {
        return self.value;
    }

    pub fn get_kight_num(&self) -> i32 {
        return self.kight;
    }

    pub fn get_card(&self, index:usize) -> String {
        return self.hand.get(index).unwrap().to_string();
    }

    pub fn dic_kight(&mut self) {
        self.kight -= 1;
    }
}