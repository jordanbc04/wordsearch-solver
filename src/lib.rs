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
#[derive(Debug, PartialEq, Eq, EnumIter, Copy, Clone)]
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

impl WordDirection {
    pub fn get_relative_x_y(self) -> (i8, i8) {

        match self {
            WordDirection::Up => (0, 1),
            WordDirection::UpRight => (1, 1),
            WordDirection::Right => (1, 0),
            WordDirection::DownRight => (1, -1),
            WordDirection::Down => (0, -1),
            WordDirection::DownLeft => (-1, -1),
            WordDirection::Left => (-1, 0),
            WordDirection::UpLeft => (-1, 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CrosswordWordData {
    position: CrosswordPosition,
    direction: WordDirection
}
#[derive(Debug, PartialEq, Eq)]
pub struct CrosswordPuzzleSolver {
    pub crossword: Vec<Vec<char>>,
    letter_index: HashMap<char, Vec<CrosswordPosition>>,
    cached_answers: HashMap<String, Option<CrosswordWordData>>
}

impl CrosswordPuzzleSolver {
    pub fn new(crossword: String, dimensions: CrosswordDimensions) -> Result<Self, CrosswordError> {
        if crossword.len() != (dimensions.height * dimensions.width) {
            return Err(CrosswordError::InvalidInput("crossword actual data length does not match dimensions implied length"));
        }
        
        let mut letter_index: HashMap<char, Vec<CrosswordPosition>> = HashMap::new();

        for (x, char) in crossword.chars().collect::<Vec<char>>().chunks(dimensions.width).flatten().enumerate() {
            let position = CrosswordPosition {
                x: x as isize % dimensions.width as isize,
                y: x as isize / dimensions.width as isize
            };

            if (position.x == 0 || position.x == 1) && position.y == 0 {
                dbg!(position);
                dbg!(char);
            }

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
                .collect::<Vec<Vec<char>>>(),
            letter_index,
            cached_answers: HashMap::new()
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

    pub fn find_word(&mut self, word: &str) -> Result<Option<CrosswordWordData>, CrosswordError> {
        if let Some(answer) = self.cached_answers.get(word) {
            return Ok(*answer)
        }
        println!("looking for {}", &word);
        let first_char = word.chars().next().unwrap();
        if let Some(positions) = self.letter_index.get(&first_char) {
            '_position_loop: for position in positions {
                println!("trying position {:?}", position);
                'direction_loop: for direction in WordDirection::iter() {
                    println!("trying direction {:?}", &direction);
                    '_word_loop: for (index, expected_letter) in word.chars().enumerate() {
                        let crossword_letter = self.get_letter(position.get_position_from_direction(&direction, index));
                        if crossword_letter.is_none() || *crossword_letter.unwrap() != expected_letter {
                            println!("{:?} (at position {:?}) does not match expected {:?}", crossword_letter, position.get_position_from_direction(&direction, index), expected_letter);
                            continue 'direction_loop;
                        }
                        println!("{} matched", crossword_letter.unwrap());
                    }
                    //dbg!("word found!");
                    let result = Some(CrosswordWordData {
                        position: *position,
                        direction
                    });
                    dbg!(&result);
                    self.cached_answers.insert(word.to_owned(), result);
                    return Ok(result);
                }

            }
        }
        self.cached_answers.insert(word.to_owned(), None);
        Ok(None)
    }

    fn calculate_colored_coordinates(&self) -> HashMap<(isize, isize), i32>{
        let mut colored_positions = HashMap::new();
        for (word, result) in &self.cached_answers {
            if result.is_none() {
                continue;
            }
            let result = result.unwrap();

            let origin_x = result.position.x;
            let origin_y = result.position.y;

            let (x_direction_multiplier, y_direction_multiplier) = result.direction.get_relative_x_y();
            let direction_key = match result.direction {
                WordDirection::Up => 1,
                WordDirection::UpRight => 2,
                WordDirection::Right => 3,
                WordDirection::DownRight => 4,
                WordDirection::Down => 5,
                WordDirection::DownLeft => 6,
                WordDirection::Left => 7,
                WordDirection::UpLeft => 8,
            };

            for n in 0..word.len() as isize {
                colored_positions.insert((origin_x + (x_direction_multiplier as isize * n), origin_y + (y_direction_multiplier as isize * n)), direction_key);
            }
        }

        colored_positions
    }

    pub fn pretty_print(&mut self, words: Option<Vec<&str>>) -> String {
        if let Some(words) = words {
            for word in words {
                _ = self.find_word(word);
            }
        }
        let RED = "\x1B[38:5:196m";
        let ORANGE = "\x1B[38:5:202m";
        let YELLOW= "\x1B[38:5:226m";
        let GREEN= "\x1B[38:5:76m";
        let BLUE= "\x1B[38:5:39m";
        let PURPLE= "\x1B[38:5:63m";
        let PINK = "\x1B[38:5:201m";
        let WHITE = "\x1B[38:5:231m";

        let RESET = "\x1B[38:5:244m";
        let mut output = String::new();
        let colored_coords = self.calculate_colored_coordinates();

        println!("{} ↖{} ↑{} ↗{}", WHITE, RED, ORANGE, RESET);
        println!("{} ←  {} →{}", PINK, YELLOW, RESET);
        println!("{} ↙{} ↓{} ↘{}", PURPLE, BLUE, GREEN, RESET);

        for (y, row) in self.crossword.iter().enumerate().rev() {
            output += "\t";
            for (x, char) in row.iter().enumerate() {
                if colored_coords.contains_key(&(x as isize, y as isize)) {
                    output += match colored_coords[&(x as isize, y as isize)] {
                        1 => RED,
                        2 => ORANGE,
                        3 => YELLOW,
                        4 => GREEN,
                        5 => BLUE,
                        6 => PURPLE,
                        7 => PINK,
                        8 => WHITE,
                        _ => panic!("FUCK!!!!!!!!!!!!!!")
                    };
                }
                output.push(*char);
                if colored_coords.contains_key(&(x as isize, y as isize)) {
                    output += RESET;
                }
                output += " ";
            }
            output += "\n";
        }

        output
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

    let mut solver = solver.unwrap();
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