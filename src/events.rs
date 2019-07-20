//extern crate sdl2;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
use crate::constants::Keyboard;

pub struct SnakeEvent {
    key_code: u32
}

impl SnakeEvent {
        pub fn new(key_code: u32) -> SnakeEvent {
        SnakeEvent {
            key_code,
        }
    }
    pub fn get_key(&self) -> Keyboard {

                match self.key_code{
            38 => {
                Keyboard::Up
            }
            40 => {
                Keyboard::Down
            }
            37 => {
                Keyboard::Left
            }
            39 => {
                Keyboard::Right
            }
            _ => {
                Keyboard::Unknown
            }
        }
    }
}