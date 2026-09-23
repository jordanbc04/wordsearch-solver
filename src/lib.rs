#![cfg(target_pointer_width = "64")]

use std::{collections::HashMap, ops::Add};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone)]
pub enum CrosswordError {
    InvalidInput(&'static str)

}
pub struct CrosswordDimensions { pub width: usize, pub height: usize }
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CrosswordPosition { pub x: isize, pub y: isize }
impl CrosswordPosition {
    fn get_position_from_direction(self, direction: &WordDirection, amount: usize) -> Self {
        match direction {
            WordDirection::Up => CrosswordPosition { x: self.x, y: self.y + amount as isize },
            WordDirection::UpRight => CrosswordPosition { x: self.x + amount as isize, y: self.y + amount as isize },
            WordDirection::Right => CrosswordPosition { x: self.x + amount as isize, y: self.y },
            WordDirection::DownRight => CrosswordPosition { x: self.x + amount as isize, y: self.y - amount as isize },
            WordDirection::Down => CrosswordPosition { x: self.x, y: self.y - amount as isize },
            WordDirection::DownLeft => CrosswordPosition { x: self.x - amount as isize, y: self.y - amount as isize },
            WordDirection::Left => CrosswordPosition { x: self.x - amount as isize, y: self.y },
            WordDirection::UpLeft => CrosswordPosition { x: self.x - amount as isize, y: self.y + amount as isize },
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
#[derive(Debug, PartialEq, Eq, EnumIter)]
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
pub struct CrosswordWordData {
    position: CrosswordPosition,
    direction: WordDirection
}

#[derive(Debug, PartialEq, Eq)]
pub struct CrosswordPuzzleSolver {
    pub crossword: Vec<Vec<char>>,
    letter_index: HashMap<char, Vec<CrosswordPosition>>,
    cached_answers: HashMap<String, CrosswordWordData>
}

impl CrosswordPuzzleSolver {
    pub fn new(crossword: String, dimensions: CrosswordDimensions) -> Result<Self, CrosswordError> {
        if crossword.len() != (dimensions.height * dimensions.width) {
            return Err(CrosswordError::InvalidInput("crossword actual data length does not match dimensions implied length"));
        }
        
        let mut letter_index: HashMap<char, Vec<CrosswordPosition>> = HashMap::new();

        for (x, char) in crossword.chars().collect::<Vec<char>>().chunks(dimensions.width).rev().flatten().enumerate() {
            let position = CrosswordPosition {
                x: x as isize % dimensions.width as isize,
                y: x as isize / dimensions.width as isize
            };
            //dbg!(position);
            //dbg!(char);

            if let Some(positions) = letter_index.get_mut(char) {
                positions.push(position);
            } else {
                letter_index.insert(*char, vec![position]);
            }
        }

        Ok(Self {
            crossword: crossword
                .chars()
                .collect::<Vec<char>>()
                .chunks(dimensions.width)
                .map(|x| x.to_vec())
                .rev()
                .collect::<Vec<Vec<char>>>(),
            letter_index,
        })
    }

    pub fn square_puzzle(crossword: String) -> Result<Self, CrosswordError> {
        let crossword_length_float = f64::from(crossword.len() as u32);
        let crossword_side_length = crossword_length_float.sqrt();

        if crossword_side_length.fract() != 0.0 {
            Err(CrosswordError::InvalidInput("crossword data is not square"))
        } else {
            Self::new(crossword, CrosswordDimensions {width: crossword_side_length as usize, height: crossword_side_length as usize })
        }
    }

    fn get_letter(&self, position: CrosswordPosition) -> Option<&char> {
        if position.x < 0 || position.y < 0 {
            return None;
        }
        if let Some(row) = self.crossword.get(position.y as usize) {
            row.get(position.x as usize)
        } else {
            None
        }
    }

    pub fn find_word(&self, word: &str) -> Result<Option<CrosswordWordData>, CrosswordError> {
        //dbg!("looking for {}", &word);
        let first_char = word.chars().next().unwrap();
        if let Some(positions) = self.letter_index.get(&first_char) {
            '_position_loop: for position in positions {
                //dbg!("trying position {:?}", position);
                'direction_loop: for direction in WordDirection::iter() {
                    //dbg!("trying direction {:?}", &direction);
                    '_word_loop: for (index, expected_letter) in word.chars().enumerate() {
                        let crossword_letter = self.get_letter(position.get_position_from_direction(&direction, index));
                        if crossword_letter.is_none() || *crossword_letter.unwrap() != expected_letter {
                            //dbg!("{:?} (at position {:?}) does not match expected {:?}", crossword_letter, position.get_position_from_direction(&direction, index), expected_letter);
                            continue 'direction_loop;
                        }
                        //dbg!("{} matched", crossword_letter.unwrap());
                    }
                    ////dbg!("word found!");
                    return Ok(Some(CrosswordWordData {
                        position: *position,
                        direction
                    }))
                }

            }
        }
        Ok(None)
    }
}

#[test]
fn create_crossword_solver() {
    let crossword = String::from("WVFXZYZWGXDEPARAGUAYLSVMEREOIUJUBJEAWGNIDBYSGYNNUECPYAMALUBIINHPERUONORRNTGDBCOCLASUAIUWTKHIEOUSDNIEOIVPXYCQLAABLIZARBFUYINEACFAKUCKXGALEUZENEVZ");
    let solver = CrosswordPuzzleSolver::new(crossword, CrosswordDimensions { width: 12, height: 12 });
    
    assert!(solver.is_ok());
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

#[test]
fn search_for_a_word() {
    let crossword = String::from("WVFXZYZWGXDEPARAGUAYLSVMEREOIUJUBJEAWGNIDBYSGYNNUECPYAMALUBIINHPERUONORRNTGDBCOCLASUAIUWTKHIEOUSDNIEOIVPXYCQLAABLIZARBFUYINEACFAKUCKXGALEUZENEVZ");
    let solver = CrosswordPuzzleSolver::new(crossword, CrosswordDimensions { width: 12, height: 12 });
    assert!(solver.is_ok());

    let solver = solver.unwrap();
    let first_word = String::from("PARAGUAY");
    assert!(solver.find_word(&first_word).is_ok());
    assert!(solver.find_word(&first_word).unwrap().is_some());
    assert_eq!(
        solver.find_word(&first_word).unwrap().unwrap(),
        CrosswordWordData { direction: WordDirection::Right, position: CrosswordPosition { x: 0, y: 10 }}
    );

    let second_word = String::from("PERU");

    assert!(solver.find_word(&second_word).is_ok());
    assert!(solver.find_word(&second_word).unwrap().is_some());
    assert_eq!(
        solver.find_word(&second_word).unwrap().unwrap(),
        CrosswordWordData { direction: WordDirection::Right, position: CrosswordPosition { x: 3, y: 6 }}
    );
}