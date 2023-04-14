use std::collections::HashSet;

use crate::dictionary::word_exists;

const LETTERS_PER_SIDE: usize = 4;

type LetterCoord = (usize, usize);
type Side = [char; LETTERS_PER_SIDE];
type Triangle = [Side; 3];

#[derive(Debug)]
pub(crate) struct Solution {
    points: Vec<LetterCoord>,
    words: Vec<String>,
    triangle: Triangle,
    begin: usize,
}

impl Solution {
    pub fn new(triangle: Triangle) -> Self {
        Self {
            points: vec![],
            words: vec![],
            triangle,
            begin: 0,
        }
    }

    pub fn add_letter(&mut self, point: LetterCoord) -> bool {
        let is_same_side = self
            .points
            .last()
            .map(|(side, _)| side == &point.0)
            .unwrap_or_default();

        if is_same_side {
            return false;
        }

        self.points.push(point);

        true
    }

    pub fn enter_word(&mut self) -> bool {
        let word = self.get_word();

        if !word_exists(&word) {
            return false;
        }

        self.begin = self.points.len();
        self.words.push(word);

        true
    }

    pub fn all_letters_used(&self) -> bool {
        let used_letters = self.points.iter().collect::<HashSet<&LetterCoord>>();

        used_letters.len() == LETTERS_PER_SIDE * 3
    }

    fn get_word(&self) -> String {
        let mut res = String::with_capacity(self.points.len());

        for point in self.points.iter().skip(self.begin) {
            let letter = self.triangle[point.0][point.1];
            res.push(letter);
        }

        res
    }
}
