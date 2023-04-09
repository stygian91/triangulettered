const LETTERS_PER_SIDE: usize = 4;

type LetterCoord = (usize, usize);
type Side = [char; LETTERS_PER_SIDE];
type Triangle = [Side; 3];

fn word_exists(word: &str) -> bool {
    // TODO: implement dictionary search
    true
}

#[derive(Debug)]
struct Solution {
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

    fn get_word(&self) -> String {
        let mut res = String::with_capacity(self.points.len());

        for point in self.points.iter().skip(self.begin) {
            let letter = self.triangle[point.0][point.1];
            res.push(letter);
        }

        res
    }
}

fn main() {
    let a = ['t', 'n', 'w', 'u'];
    let b = ['m', 'y', 'o', 'p'];
    let c = ['q', 'a', 'i', 's'];
    let mut solution = Solution::new([a, b, c]);
    solution.add_letter((0, 0));
    solution.add_letter((2, 1));
    solution.add_letter((0, 1));
    solution.enter_word();
    println!("{solution:#?}");
}
