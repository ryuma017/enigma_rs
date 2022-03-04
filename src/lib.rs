mod enigma;
mod interiors;
mod wheels;

pub use interiors::{PlugBoard, Rotor, Reflector};
pub use enigma::Enigma;


trait Index {
    fn letter_index(&self) -> usize;

    fn map_index(&self, map: &[char]) -> usize;
}

impl Index for char {
    fn letter_index(&self) -> usize {
        *self as usize - 65
    }

    fn map_index(&self, map: &[char]) -> usize {
        map.iter().position(|c| *c == *self).unwrap()
    }
}

trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for usize {
    fn to_char(&self) -> char {
        ((*self % 26) as u8 + 65) as char
    }
}
