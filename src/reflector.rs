const ALPHABETH: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Reflector {
    wiring: [char; 26],
}

impl Reflector {
    pub fn new(inputs: [char; 26]) -> Self {
        Reflector { wiring: inputs }
    }

    pub fn reflect(&mut self, input_letter: char) -> char {
        let letter_index = Self::get_letter_index(input_letter); //nomral alphabeth
        if letter_index > ALPHABETH.len() {
            return self.wiring[0];
        }
        self.wiring[letter_index]
    }
    fn get_letter_index(letter: char) -> usize {
        ALPHABETH
            .iter()
            .position(|&x| x == letter)
            .unwrap_or_else(|| 100)
    }
}

#[cfg(test)]
mod test {
    use super::Reflector;

    #[test]
    fn test() {
        let reflector: [char; 26] = [
            'E', 'J', 'M', 'Z', 'A', 'L', 'Y', 'X', 'V', 'B', 'W', 'F', 'C', 'R', 'Q', 'U', 'O',
            'N', 'T', 'S', 'P', 'I', 'K', 'H', 'G', 'D',
        ];

        let mut reflector = Reflector::new(reflector);
        print!("A->{}", reflector.reflect('A'));
    }
}
