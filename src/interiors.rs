use crate::ToChar;
use crate::Index;
use crate::wheels::{ROTORS, REFLECTORS};


//--- PlugBoard ---//
pub struct PlugBoard {
    wiring_map: Vec<char>,
}

impl PlugBoard {
    pub fn set(pairs: &str) -> Self {
        // wire pairs
        let mut wiring_map: Vec<char> = ('A'..='Z').collect();

        for pair in pairs
            .split_whitespace()
            .map(|pair| pair.chars().collect::<Vec<char>>())
        {
            wiring_map[pair[0].letter_index()] = pair[1];
            wiring_map[pair[1].letter_index()] = pair[0];
        }

        Self {
            wiring_map,
        }
    }

    pub fn convert(&self, index: usize) -> usize {
        let char = index.to_char();
        char.map_index(&self.wiring_map)
    }
}

//--- Rotor ---//
pub struct Rotor {
    letters: Vec<char>,
    wiring: Vec<char>,
    notch: char,
}

impl Rotor {
    // pub fn set(wheel: &str, initial_position: char, offset: char) -> Self {
    //     let mut letters = letters();
    //     let (mut wiring, notch) = Rotor::choose(wheel);
}

//--- Reflector ---//
pub struct Reflector {}

impl Reflector {}


#[cfg(test)]
mod tests {
    use super::{PlugBoard, Rotor, Reflector};

    #[test]
    fn no_plugs() {
        let plugboard = PlugBoard::set("");
        assert_eq!(plugboard.convert(0), 0);
    }

    #[test]
    fn multiple_plugs() {
        let plugboard = PlugBoard::set("AC BD EF");
        println!("{:?}", ('A'..'Z').collect::<Vec<char>>());
        println!("{:?}", plugboard.wiring_map);
        assert_eq!(plugboard.convert(1), 3);
        assert_eq!(plugboard.convert(2), 0);
        assert_eq!(plugboard.convert(5), 4);
    }
}