#[cfg(target_pointer_width = "64")]

use std::fmt::Error;
use std::{collections::HashMap, ops::Add};

use crate::WordDirection::Right;

#[derive(Debug, Clone)]
enum CrosswordError {
    InvalidInput(&'static str)

}
struct CrosswordDimensions { pub width: usize, pub height: usize }
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CrosswordPosition { pub x: usize, pub y: usize }
impl CrosswordPosition {
    fn get_position_from_direction(self, direction: WordDirection, amount: usize) -> Self {
        match direction {
            WordDirection::Up => todo!(),
            WordDirection::UpRight => todo!(),
            WordDirection::Right => CrosswordPosition { x: self.x + (amount), y: self.y },
            WordDirection::DownRight => todo!(),
            WordDirection::Down => todo!(),
            WordDirection::DownLeft => todo!(),
            WordDirection::Left => todo!(),
            WordDirection::UpLeft => todo!(),
        }

    }
}
impl Add for CrosswordPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum WordDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Debug, PartialEq, Eq)]
struct CrosswordWordData {
    position: CrosswordPosition,
    direction: WordDirection
}

#[derive(Debug, PartialEq, Eq)]
struct CrosswordPuzzleSolver {
    crossword: Vec<Vec<char>>,
    letter_index: HashMap<char, Vec<CrosswordPosition>>
}

impl CrosswordPuzzleSolver {
    fn new(crossword: String, dimensions: CrosswordDimensions) -> Result<Self, CrosswordError> {
        if crossword.len() != (dimensions.height * dimensions.width) {
            return Err(CrosswordError::InvalidInput("crossword actual data length does not match dimensions implied length"));
        }
        
        let mut letter_index: HashMap<char, Vec<CrosswordPosition>> = HashMap::new();

        let mut x = 0;
        for char in crossword.chars() {
            let position = CrosswordPosition {
                x: x % dimensions.width as usize,
                y: x / dimensions.width as usize
            };

            if let Some(positions) = letter_index.get_mut(&char) {
                positions.push(position);
            } else {
                letter_index.insert(char.clone(), vec![position]);
            }
            x += 1;
        }

        return Ok(Self {
            crossword: crossword
                .chars()
                .collect::<Vec<char>>()
                .chunks(dimensions.width as usize)
                .map(|x| x.to_vec())
                .collect::<Vec<Vec<char>>>(),
            letter_index,
        })
    }

    fn square_puzzle(crossword: String) -> Result<Self, CrosswordError> {
        let crossword_length_float = f64::from(crossword.len() as u32);
        let crossword_side_length = crossword_length_float.sqrt();

        if crossword_side_length.fract() != 0.0 {
            Err(CrosswordError::InvalidInput("crossword data is not square"))
        } else {
            Self::new(crossword, CrosswordDimensions {width: crossword_side_length as usize, height: crossword_side_length as usize })
        }
    }

    fn get_letter(&self, position: CrosswordPosition) -> char {
        self.crossword.get(position.y).unwrap().get(position.x).unwrap().clone()
    }

    pub fn search_word(&self, word: String) -> Result<Option<CrosswordWordData>, CrosswordError> {
        let first_char = word.chars().next().unwrap();
        if let Some(positions) = self.letter_index.get(&first_char) {
            'position_loop: for position in positions {
                'word_loop: for (index, letter) in word.chars().enumerate() {
                    if self.get_letter(position.get_position_from_direction(WordDirection::Right, index)) != letter {
                        continue 'position_loop;
                    }
                }

                return Ok(Some(CrosswordWordData {
                    position: position.clone(),
                    direction: WordDirection::Right
                }))
            }
        }

        Ok(None)
    }
}

fn main() {
}

#[test]
fn create_crossword_solver() {
    let crossword = String::from("WVFXZYZWGXDEPARAGUAYLSVMEREOIUJUBJEAWGNIDBYSGYNNUECPYAMALUBIINHPERUONORRNTGDBCOCLASUAIUWTKHIEOUSDNIEOIVPXYCQLAABLIZARBFUYINEACFAKUCKXGALEUZENEVZ");
    let solver = CrosswordPuzzleSolver::new(crossword, CrosswordDimensions { width: 12, height: 12 });
    
    assert!(solver.is_ok());
}

#[test]
fn search_for_a_word() {
    let crossword = String::from("WVFXZYZWGXDEPARAGUAYLSVMEREOIUJUBJEAWGNIDBYSGYNNUECPYAMALUBIINHPERUONORRNTGDBCOCLASUAIUWTKHIEOUSDNIEOIVPXYCQLAABLIZARBFUYINEACFAKUCKXGALEUZENEVZ");
    let solver = CrosswordPuzzleSolver::new(crossword, CrosswordDimensions { width: 12, height: 12 });
    assert!(solver.is_ok());
    let solver = solver.unwrap();
    assert!(solver.search_word(String::from("PARAGUAY")).is_ok());
    assert!(solver.search_word(String::from("PARAGUAY")).unwrap().is_some());
    assert_eq!(
        solver.search_word(String::from("PARAGUAY")).unwrap().unwrap(),
        CrosswordWordData { direction: WordDirection::Right, position: CrosswordPosition { x: 0, y: 1 }}
    );

    assert!(solver.search_word(String::from("PERU")).is_ok());
    assert!(solver.search_word(String::from("PERU")).unwrap().is_some());
    dbg!(solver.search_word(String::from("PERU")).unwrap().unwrap());
    assert_eq!(
        solver.search_word(String::from("PERU")).unwrap().unwrap(),
        CrosswordWordData { direction: WordDirection::Right, position: CrosswordPosition { x: 3, y: 5 }}
    );

    assert!(solver.search_word(String::from("asdf!")).is_ok());
}

#[test]
fn square_puzzle_equals_normal_puzzle() {
    let crossword = String::from("WVFXZYZWGXDEPARAGUAYLSVMEREOIUJUBJEAWGNIDBYSGYNNUECPYAMALUBIINHPERUONORRNTGDBCOCLASUAIUWTKHIEOUSDNIEOIVPXYCQLAABLIZARBFUYINEACFAKUCKXGALEUZENEVZ");
    let solver_normal = CrosswordPuzzleSolver::new(crossword.clone(), CrosswordDimensions { width: 12, height: 12 });
    assert!(solver_normal.is_ok());
    
    let solver_square = CrosswordPuzzleSolver::square_puzzle(crossword.clone());
    assert!(solver_square.is_ok());

    assert_eq!(solver_normal.unwrap(), solver_square.unwrap());
}