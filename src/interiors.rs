use crate::{ToChar, Index};
use crate::wheels::{ROTORS, REFLECTORS};


//--- PlugBoard ---//
pub struct PlugBoard {
    wiring: Vec<char>,
}

impl PlugBoard {
    pub fn set(pairs: &str) -> Self {
        // wire pairs
        let mut wiring: Vec<char> = ('A'..='Z').collect();

        for pair in pairs
            .split_whitespace()
            .map(|pair| pair.chars().collect::<Vec<char>>())
        {
            wiring[pair[0].letter_index()] = pair[1];
            wiring[pair[1].letter_index()] = pair[0];
        }

        Self {
            wiring,
        }
    }

    pub fn convert(&self, index: usize) -> usize {
        let char = index.to_char();
        char.map_index(&self.wiring)
    }
}

//--- Rotor ---//
pub struct Rotor {
    letters: Vec<char>,
    wiring: Vec<char>,
    notch: char,
}

impl Rotor {
    pub fn set(wheel: &str, initial_position: char, offset: char) -> Self {
        let i = match wheel {
            "I" => 0,
            "II" => 1,
            "III" => 2,
            "IV" => 3,
            "V" => 4,
            _ => panic!("choose one rotor from [I, II, III, IV, V]"),
        };
        let mut letters: Vec<char> = ('A'..='Z').collect();
        let (mut wiring, notch): (Vec<char>, char) = {
            let rotor = ROTORS[i];
            (rotor.0.chars().collect(), rotor.1)
        };

        Self {
            letters,
            wiring,
            notch,
        }
    }
}

//--- Reflector ---//
pub struct Reflector {
    wiring: Vec<char>,
}

impl Reflector {
    pub fn set(wheel: &str) -> Self {
        let i = match wheel {
            "UKW-A" => 0,
            "UKW-B" => 1,
            "UKW-C" => 2,
            _ => panic!("choose one reflector from [UKW-A, UKW-B, UKW-C]")
        };

        let wiring: Vec<char> = REFLECTORS[i].chars().collect();

        Self {
            wiring
        }
    }

    pub fn reflect(&self, index: usize) -> usize {
        let char = index.to_char();
        char.map_index(&self.wiring)
    }
}


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
        println!("{:?}", plugboard.wiring);
        assert_eq!(plugboard.convert(1), 3);
        assert_eq!(plugboard.convert(2), 0);
        assert_eq!(plugboard.convert(5), 4);
    }
}