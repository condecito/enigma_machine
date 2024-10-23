use board::Board;
use enigma::Enigma;
use reflector::Reflector;
use rotor::Rotor;

mod board;
mod enigma;
mod reflector;
mod rotor;
#[allow(warnings)]
fn main() {
    let reflector: [char; 26] = [
        'E', 'J', 'M', 'Z', 'A', 'L', 'Y', 'X', 'V', 'B', 'W', 'F', 'C', 'R', 'Q', 'U', 'O', 'N',
        'T', 'S', 'P', 'I', 'K', 'H', 'G', 'D',
    ];

    let rotor_wiring1: [char; 26] = [
        'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U',
        'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J',
    ];

    const rotor_wiring2: [char; 26] = [
        'E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y', 'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N', 'F',
        'T', 'G', 'K', 'D', 'C', 'M', 'W', 'B',
    ];

    let rotor1 = Rotor::new(rotor_wiring1, 'J');
    let rotor2 = Rotor::new(rotor_wiring2, 'S');
    let reflector = Reflector::new(reflector);
    let board = Board::new(&[('A', 'B'), ('C', 'D')]);
    let rotors = vec![rotor1, rotor2];
    let mut engima = Enigma::new(rotors, board, reflector, false);

    // engima.print_rotors();

    let message = "HOLA"; //"VLOQ";
    let encoded_message: String = message.chars().map(|c| engima.encode(c)).collect();
    println!("Encoded message: {}", encoded_message);
}
