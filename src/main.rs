use std::{error::Error, fs::File, io::Read, path::Path};
use crossword_solver;

struct ArgumentParseError {}
struct Settings {
    pub wordlist: Vec<String>,
    pub crossword: String,
    pub crossword_height: usize,
    pub crossword_width: usize
}

fn parse_args(args: Vec<String>) -> Result<Settings, Box<dyn Error>> {
    let mut x: usize = 0;
    
    let mut raw_wordlist = String::new();
    let mut wordlist = vec![];
    let mut crossword = String::new();
    let mut crossword_height = 0 as usize;
    let mut crossword_width = 0 as usize;
    while x < args.len() {
        let current_arg = args.get(x).unwrap();
        match current_arg.as_ref() {
            "--wordlist" => {
                if let Some(next_arg) = args.get(x+1) {
                    let wordlist_path = Path::new(next_arg);
                    if wordlist_path.exists() {
                        let mut wordlist_contents = File::options().read(true).open(next_arg)?;
                        wordlist_contents.read_to_string(&mut raw_wordlist)?;
                        wordlist = raw_wordlist.split('\n').collect::<Vec<&str>>();
                        x += 1;
                    } else {
                        panic!("wordlist path does not lead to an existing file")
                    }
                } else {
                    panic!("expected a name or a path to a file after argument \"--wordlist\"")
                }
            }
            "--crossword" => {
                if let Some(next_arg) = args.get(x+1) {
                    let crossword_path = Path::new(next_arg);
                    if crossword_path.exists() {
                        let mut crossword_contents = File::options().read(true).open(crossword_path)?;
                        let mut raw_crossword = String::new();
                        crossword_contents.read_to_string(&mut raw_crossword)?;

                        let mut chars = raw_crossword.chars().peekable();
                        loop {
                            let char = chars.next().unwrap();
                            match char {
                                '\n' => {
                                    crossword_height += 1;
                                    break;
                                }
                                _ => crossword_width += 1
                            }
                        }

                        loop {
                            let char = chars.next();
                            if char.is_none() {
                                crossword_height += 1;
                                break
                            }
                            let char = char.unwrap();
                            if char == '\n' {
                                if chars.peek().is_some() {
                                    crossword_height += 1;
                                } else {
                                    break;
                                }
                            }
                        }

                        crossword = raw_crossword.chars().filter(|x| *x != '\n').collect::<String>();

                        x += 1;
                    } else {
                        panic!("crossword path does not lead to an existing file")
                    }
                } else {
                    panic!("expected a name or a path to a file after argument \"--crossword\"")
                }
            }
            _ => {
                panic!("invalid argument \"{}\"", current_arg);
            }
        }

        x += 1;
    }

    let s = Settings {
        crossword,
        crossword_height,
        crossword_width,
        wordlist: wordlist.into_iter().map(|x| x.to_owned()).collect()
    };

    Ok(s)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let settings = parse_args(args)?;

    let missing_wordlist = settings.wordlist.is_empty();
    let missing_crossword = settings.crossword == String::new();
    if missing_wordlist && missing_crossword {
        panic!("expected arguments \"--wordlist\" and \"--crossword\"")
    }
    else if missing_wordlist {
        panic!("expected argument \"--wordlist\"")
    }
    else if missing_crossword {
        panic!("expected argument \"--crossword\"")
    }

    let crossword_solver = crossword_solver::CrosswordPuzzleSolver::new(settings.crossword, crossword_solver::CrosswordDimensions {height: settings.crossword_height, width: settings.crossword_width}).unwrap();
    for word in settings.wordlist {
        dbg!(String::from("finding word ") + &word);
        dbg!(crossword_solver.find_word(&word));
    }

    Ok(())
}

#[test]
fn test_read_file() {

}