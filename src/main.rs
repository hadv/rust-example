use ethereum_types::U512;
use std::ops::{Shl};

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!()
    }
}

fn set_compact(compact: u32) -> U512 {
    let size = compact >> 24;
    let mut word = compact & 0x007fffff;
    if size <= 3 {
        word >>= 8 * (3 - size);
        U512::from(word)
    } else {
        U512::from(word).shl(8 * (size - 3))
    }
}

fn main() {
    let work = set_compact(0x18abcdef);
    println!("{}", work);
}
