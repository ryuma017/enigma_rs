use enigma_i::PlugBoard;
use enigma_i::letters;

fn main() {
    let plugboard = PlugBoard::set("AC BD");
    let letters = letters();
    let text = "BADCFE";
    for c in text.chars() {
        let mut index = letters.iter().position(|char| *char == c).unwrap();
        index = plugboard.convert_backward(index);
        index = plugboard.convert_backward(index);
        println!("{}", letters[index]);
    }
}