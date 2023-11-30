use std::{io, fmt::Display, fmt::Debug, error::Error};
use rand::{self, Rng};

struct Life {
    grid: Vec<bool>,
    x_size: u32,
    y_size: u32,

    alive_char: char,
    dead_char: char,

    alive_behavior: Vec<bool>,
    dead_behavior: Vec<bool>,

}

#[allow(dead_code)]
enum State {
    Empty,
    Full,
    Rand (u32),
}
#[derive(Debug)]
enum GameOfLifeError {
    BoardTooSmall,
}

impl Error for GameOfLifeError {
    
}

impl Display for GameOfLifeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::BoardTooSmall => "The board cannot be less than 3 units tall or wide"
        };
        write!(f, "Error: {message}")
    }
}

impl Life {
    fn new(sx: u32, sy: u32) -> Result<Life, GameOfLifeError> {
        if sx >= 3 && sy >= 3 {
            let mygrid: Vec<bool> = Vec::with_capacity((sx*sy).try_into().unwrap());
            let myliving: Vec<bool> = vec![false, false, false, true, true, false, false, false, false];
            let mydead: Vec<bool> = vec![false, false, false, true, false, false, false, false, false];

            Ok(
                Life { 
                    grid: mygrid, 
                    x_size: sx, 
                    y_size: sy, 
                    alive_char: '@', 
                    dead_char: ' ',
                    alive_behavior: myliving,
                    dead_behavior: mydead,

                }   
            )
        }
        else {
            Err(GameOfLifeError::BoardTooSmall)
        }
    }

    fn populate(&mut self, desired_state: State) {
        match desired_state {
            State::Empty => {
                self.grid = vec!(false; (self.x_size * self.y_size).try_into().unwrap());
            }
            State::Full => { 
                self.grid = vec!(true; (self.x_size * self.y_size).try_into().unwrap());
            }
            State::Rand(bar) => {
                self.grid = vec!(false; (self.x_size * self.y_size).try_into().unwrap());

                for i in 0..self.grid.len() as u32 {
                    let num: u32 = rand::thread_rng().gen_range(1..100);
                    if num > bar {
                        self.grid[<u32 as TryInto<usize>>::try_into(i).unwrap()] = true;
                    }
                }
            }
        }
    }

    fn to_index(&self, cords: (u32, u32)) -> u32 { 
        if cords == (0,0) {
            return 0;
        }
        let index: u32 = cords.0 + (cords.1 * self.x_size);
        index
    }

    fn to_cord(&self, index: u32) -> (u32, u32) {
        if index == 0 {
            return (0, 0);
        }

        let cords: (u32, u32) = (
            index % self.x_size, 
            index / self.x_size);
        cords

    }

    fn display(&self) {
        for i in 0..self.y_size {
            let mut this_str = String::new();
            for ii in 0..self.x_size {
                if self.grid[<u32 as TryInto<usize>>::try_into(self.to_index((ii, i))).unwrap()] {
                    this_str.push(self.alive_char);
                    this_str.push(' ');
                }
                else {
                    this_str.push(self.dead_char);
                    this_str.push(' ');
                }
            }
            println!("{this_str}");
        }
    }

    fn get_cell_count(&self, i: u32) -> u8 {
            let mut count: u8 = 0;
            for x in 0..3 {
                for y in 0..3 {
                    if x == 1 && y == 1 {
                        continue;
                    }
                    let mut my_cords = self.to_cord(i);
                    if x == 0 && self.to_cord(i).0 == 0 {
                        my_cords.0 = self.x_size - 1;
                            }
                    else if x == 2 && self.to_cord(i).0 == self.x_size - 1{
                        my_cords.0 = 0;
                    }
                    else {
                        my_cords.0 = my_cords.0 + x - 1;
                    }

                    if y == 0 && self.to_cord(i).1 == 0 {
                        my_cords.1 = self.y_size - 1;
                    }
                    else if y == 2 && self.to_cord(i).1 == self.y_size - 1{
                        my_cords.1 = 0;
                    }
                    else {
                        my_cords.1 = my_cords.1 + y - 1;
                    } 
                    if self.grid[<u32 as TryInto<usize>>::try_into(self.to_index(my_cords)).unwrap()] {
                        count += 1;
                    }
                }
            }
            count
        }

    fn update(&mut self) {
        for i in 0..self.grid.len() {
            let count = self.get_cell_count(i.try_into().unwrap());
            if self.grid[i] {
                self.grid[i] = self.alive_behavior[<u8 as TryInto<usize>>::try_into(count).unwrap()];
            }
            else {
                self.grid[i] = self.dead_behavior[<u8 as TryInto<usize>>::try_into(count).unwrap()];
            }
        }
    }
}


fn main() { 
    let mut my_board: Life = match Life::new(50, 50) {
        Ok(life) => life,
        Err(error) => panic!("{error}"),
    };

    my_board.populate(State::Rand(75));

    let mut gen: u32 = 0;

    let mut w = String::new();

    loop {
        print!("{}[2J", 27 as char); // clean line
        my_board.display();
        println!("this is generation {gen}");

        io::stdin().read_line(&mut w).unwrap();

        my_board.update();
        gen += 1;
    }
}