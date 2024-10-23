use std::collections::HashSet;
use std::hash::Hash;
const ALPHABETH: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Rotor {
    alphabeth: Vec<char>,
    wiring: [char; 26],
    position: usize,
    configuration: char,
}
#[allow(warnings)]
impl Rotor {
    pub fn new(wiring: [char; 26], configuration: char) -> Self {
        Self::validate(&wiring);
        let alphabeth = Self::configure(ALPHABETH, configuration);
        Rotor {
            alphabeth: alphabeth,
            wiring,
            position: 0,
            configuration,
        }
    }

    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    fn validate(input: &[char; 26]) {
        if !Self::has_unique_elements(input.iter()) {
            print!("Warning some chars in rotor are not unique");
        }
    }

    pub fn move_rotor_forward(&mut self) {
        if self.position == self.alphabeth.len() {
            self.position = 0;
        }
        self.position += 1;
        self.alphabeth.rotate_left(1);
        self.wiring.rotate_left(1);
    }
    pub fn is_rotor_complete_cicle(&mut self) -> bool {
        self.position == self.alphabeth.len() - 1
    }

    fn configure(rotor_leter: [char; 26], configuration: char) -> Vec<char> {
        let mut vector: Vec<char> = rotor_leter.to_vec();
        if let Some(indice) = vector.iter().position(|&x| x == configuration) {
            vector.rotate_left(indice);
        } else {
            println!("Element not found.");
        }
        vector
    }

    pub fn encode_forward(&mut self, input_letter: char) -> char {
        let out = self
            .wiring
            .iter()
            .position(|&character| character == input_letter)
            .unwrap();

        self.alphabeth[out]
    }

    pub fn encode_back(&self, letter: char) -> char {
        let out = self
            .alphabeth
            .iter()
            .position(|&character| character == letter)
            .unwrap();

        self.wiring[out]
    }

    pub fn reset_position(&mut self) {
        self.position = 0;
    }

    pub fn print_rotor(&self) {
        for n in 0..self.wiring.len() {
            print!("[{}<-->{}]\n", self.alphabeth[n], self.wiring[n]);
        }
        print!("Total wiring= {}\n", &self.wiring.len());
    }
}

#[cfg(test)]
mod test {
    use super::Rotor;

    #[test]
    fn print_rotor() {
        let rotor_wiring: [char; 26] = [
            'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I',
            'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O',
        ];
        let mut rotor = Rotor::new(rotor_wiring, 'A');
        rotor.print_rotor();
    }

    #[test]
    fn rotor() {
        let rotor_wiring: [char; 26] = [
            'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I',
            'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O',
        ];
        let mut rotor = Rotor::new(rotor_wiring, 'Z');
        rotor.print_rotor();
        print!("({}<-B)\n", rotor.encode_forward('B'));
        assert_eq!('Z', rotor.encode_forward('B'));
    }
    #[test]
    fn rotor_back() {
        let rotor_wiring: [char; 26] = [
            'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I',
            'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O',
        ];
        let mut rotor = Rotor::new(rotor_wiring, 'Z');
        rotor.print_rotor();
        rotor.move_rotor_forward();
        rotor.print_rotor();
        print!("({}<-B)\n", rotor.encode_back('B'));
    }
}
