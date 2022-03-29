use std;
use rand::{Rng};

const NUMBER_OF_STEPS: usize = 8;
const NUMBER_OF_LINES: usize = 5;

enum State {
    InvalidInput,
    LostGame,
    WonGame,
    NextStep
}

struct Burmalda {
    current_step: usize,
    steps: [usize; NUMBER_OF_STEPS],
    bombs: [usize; NUMBER_OF_STEPS]
}

impl Burmalda {

    fn new() -> Burmalda {
        let mut game = Burmalda { 
            current_step: 0,
            steps: [0; NUMBER_OF_STEPS],
            bombs: [0; NUMBER_OF_STEPS]
        };
        game.generate_bombs();
        return game;
    }

    fn play(&mut self) {
        loop {
            self.print_game(PrintVariant::Normal);
            if let Some(input) = Burmalda::guess() {
                match self.check_input(input) {
                    State::InvalidInput => { },
                    State::WonGame => {
                        Burmalda::clear_terminal();
                        self.print_game(PrintVariant::Bombs);
                        println!("You won!");
                        break; 
                    },
                    State::LostGame => {
                        Burmalda::clear_terminal();
                        self.print_game(PrintVariant::Bombs);
                        println!("You lost!");
                        break;
                    },
                    State::NextStep => {
                        self.current_step += 1;
                    }
                }
            }
            Burmalda::clear_terminal();
        }
    }

    fn guess() -> Option<usize> {
        let mut input = String::new();
        if let (Ok(_), Ok(input)) = (std::io::stdin().read_line(&mut input), input.trim().parse::<usize>()) {
            Some(input)
        } else {
            None
        }
    }

    fn check_input(&mut self, input: usize) -> State {
        if input < 1 || input > NUMBER_OF_LINES {
            return State::InvalidInput;
        }

        self.steps[self.current_step] = input;

        if self.current_step == NUMBER_OF_STEPS - 1 {
            return State::WonGame;
        }

        if self.bombs[self.current_step] == input {
            return State::LostGame;
        }

        State::NextStep
    }

}

impl Burmalda {

    fn generate_bombs(&mut self) {
        for i in 0..NUMBER_OF_STEPS {
            self.bombs[i] = rand::thread_rng().gen_range(1..NUMBER_OF_LINES);
        }
    }
    
}

enum PrintVariant {
    Normal,
    Bombs
}

impl Burmalda {

    fn print_game(&self, variant: PrintVariant) {
        let mut y = 1;
        for _ in 0..NUMBER_OF_LINES {
            Burmalda::print_line();
            print!("|");
            for x in 0..NUMBER_OF_STEPS {
                let to_print = match variant {
                    PrintVariant::Normal => self.element_to_print(x, y),
                    PrintVariant::Bombs => self.element_to_print_with_bombs(x, y)
                };
                print!(" {} |", to_print);
            }
            print!("\n");
            y += 1;
        }
        Burmalda::print_line();
        print!("\n");
        Burmalda::print_coefficients();
        print!("\n");
    }

    fn print_line() {
        print!("+");
        for _ in 0..NUMBER_OF_STEPS {
            print!("---+");
        }
        print!("\n");
    }

    fn print_coefficients() {
        for x in 1..NUMBER_OF_STEPS+1 {
            print!("{}", format!("{:4.1}", x as f32 * 1.25));
        }
        println!();
    }
    
    fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }
}

impl Burmalda {

    fn element_to_print(&self, x: usize, y: usize) -> &str {
        if self.steps[x] == y { "X" } else { " " }
    }

    fn element_to_print_with_bombs(&self, x: usize, y: usize) -> &str {
        if self.bombs[x] == y { "*" } else if self.steps[x] == y { "X" } else { " " }
    }

}

fn main() {
    Burmalda::new().play()
}
