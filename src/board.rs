use std::collections::HashMap;

pub struct Board {
    chart_invert: HashMap<char, char>,
}

impl Board {
    pub fn new(invert: &[(char, char)]) -> Self {
        let mut chart_invert = HashMap::new();
        for &(v, k) in invert {
            chart_invert.insert(v, k);
            chart_invert.insert(k, v);
        }
        Self { chart_invert }
    }
    pub fn inver_char(&self, character: char) -> char {
        *self.chart_invert.get(&character).unwrap_or(&character)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board() {
        let board = Board::new(&[('A', 'B'), ('C', 'D')]);

        assert_eq!('A', board.inver_char('B'));
        assert_eq!('B', board.inver_char('A'));
        assert_eq!('C', board.inver_char('D'));
        assert_eq!('D', board.inver_char('C'));
        assert_eq!('G', board.inver_char('G'));
    }
}
