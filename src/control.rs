use std::collections::HashMap;


// 最初からイテレータだめか？？明日やる
pub fn letters() -> Vec<char> {
    vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
}

//--- PlugBoard ---//
pub struct PlugBoard {
    pub forward_map: HashMap<char, char>,
    pub backward_map: HashMap<char, char>,
}

impl PlugBoard {
    pub fn set(pairs: &str) -> Self {
        // wire pairs
        let letters = letters();
        let mut wiring = letters.clone();
        for pair in pairs.split_whitespace().map(|pair| pair.chars().collect::<Vec<char>>()) {
            wiring[letters.iter().position(|c| *c == pair[0]).unwrap()] = pair[1];
            wiring[letters.iter().position(|c| *c == pair[1]).unwrap()] = pair[0];
        }

        // set wiring map
        let forward_map: HashMap<_, _> = letters.clone().into_iter().zip(wiring.clone().into_iter()).collect();
        let backward_map: HashMap<_, _> = wiring.into_iter().zip(letters.into_iter()).collect();

        Self { forward_map, backward_map }
    }

    pub fn convert_forward(&self, index: usize) -> usize {
        let letters = letters();
        let ltr = letters[index];
        let ltr = &self.forward_map.get(&ltr).unwrap();
        letters.iter().position(|c| c == *ltr).unwrap()
    }

    pub fn convert_backward(&self, index: usize) -> usize {
        let letters = letters();
        let ltr = letters[index];
        let ltr = &self.backward_map.get(&ltr).unwrap();
        letters.iter().position(|c| c == *ltr).unwrap()
    }
}

//--- Rotor ---//
// pub struct Rotor {}

// impl Rotor {}

// //--- Reflector ---//
// pub struct Reflector {}

// impl Reflector {}
