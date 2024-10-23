use crate::{board::Board, reflector::Reflector, rotor::Rotor};

pub struct Enigma {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    board: Board,
    verbose: bool,
}
#[allow(warnings)]
impl Enigma {
    pub fn new(rotors: Vec<Rotor>, board: Board, reflector: Reflector, verbose: bool) -> Self {
        Enigma {
            rotors,
            reflector,
            board,
            verbose,
        }
    }

    pub fn print_rotors(self) {
        for rotor in self.rotors {
            rotor.print_rotor();
        }
    }
    fn log(&self, message: String) {
        if self.verbose {
            println!("[LOG] {}", message);
        }
    }

    pub fn encode(&mut self, input_letter: char) -> char {
        self.log(format!("-----------------------\n"));
        self.log(format!("char to encode {}\n", input_letter));
        let mut char_to_encode = self.board.inver_char(input_letter); //invert char
        self.log(format!("char inverted ={:?}\n", char_to_encode));
        let mut is_rotate = true;
        for index in 0..self.rotors.len() {
            self.log(format!(
                "  rotor {} char to encode {}\n",
                index, char_to_encode
            ));
            char_to_encode = self.rotors[index].encode_forward(char_to_encode);
            self.log(format!(
                "  rotor {} char to encoded {}\n",
                index, char_to_encode
            ));
            if is_rotate == true {
                self.rotors[index].move_rotor_forward();
                let is_completed = self.rotors[index].is_rotor_complete_cicle();
                self.log(format!(
                    "  rotor {} move forward, is cicle complete {}\n",
                    index, is_completed
                ));

                is_rotate = is_completed;
            }
        }
        self.log(format!("char after all rotor forward {}\n", char_to_encode));
        char_to_encode = self.reflector.reflect(char_to_encode); //reflect char
        self.log(format!("char encode on reflector={:?}\n", char_to_encode));
        for index in (0..self.rotors.len()).rev() {
            self.log(format!(
                "  rotor back {} char to encode {}\n",
                index, char_to_encode
            ));
            char_to_encode = self.rotors[index].encode_back(char_to_encode);
        }
        self.rotors[0].move_rotor_forward();
        let char_to_encode = self.board.inver_char(char_to_encode);
        self.log(format!("char  inverted={:?}\n", char_to_encode));
        self.log(format!("-----------------------!"));
        char_to_encode
    }
}
