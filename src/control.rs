use std::{collections::HashMap, char};

use super::wheels::Wheels;

pub const LETTERS: Vec<char> = vec![
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

//--- PlugBoard ---//
pub struct PlugBoard {
    wiring: String,
    forward_map: HashMap<char, char>,
    backward_map: HashMap<char, char>,
}

impl PlugBoard {
    fn set(pairs: String) -> PlugBoard {
        // wire pairs
        let mut wiring = LETTERS;
        for pair in pairs.split_whitespace(){
            let pair = pair.chars().collect::<Vec<char>>();
            wiring[LETTERS.iter().position(|c| *c == pair[0]).unwrap()] = pair[1];
            wiring[LETTERS.iter().position(|c| *c == pair[1]).unwrap()] = pair[2];
        }

        PlugBoard { wiring: (), forward_map: (), backward_map: () }
    }
}

//--- Rotor ---//
pub struct Rotor {}

impl Rotor {}

//--- Reflector ---//
pub struct Reflector {}

impl Reflector {}
