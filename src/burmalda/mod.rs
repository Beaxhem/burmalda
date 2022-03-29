mod bombs;
mod bet;

use std;

use crate::{Account, cli::CommandLine, burmalda::bombs::Bombs, burmalda::bet::Bet};

pub const NUMBER_OF_STEPS: usize = 8;
pub const NUMBER_OF_LINES: usize = 5;

enum State {
    InvalidInput,
    LostGame,
    WonGame(f32),
    NextStep
}

pub struct Burmalda {
    current_step: usize,
    steps: [usize; NUMBER_OF_STEPS],
    bombs: Bombs
}

impl Burmalda {

    pub fn new() -> Burmalda {
        let game = Burmalda { 
            current_step: 0,
            steps: [0; NUMBER_OF_STEPS],
            bombs: Bombs::new()
        };
        return game;
    }

    pub fn play(&mut self, account: &mut Account) {
        let bet = Burmalda::ask_for_bet(account);

        loop {
            self.print_game(PrintVariant::Normal);
            if let Some(input) = Burmalda::guess() {
                match self.check_input(input) {
                    State::InvalidInput => { },
                    State::WonGame(coefficient) => {
                        CommandLine::clear_terminal();
                        self.print_game(PrintVariant::Bombs);
                        println!("You won!");
                        account.balance += bet as f32 * coefficient;
                        break; 
                    },
                    State::LostGame => {
                        CommandLine::clear_terminal();
                        self.print_game(PrintVariant::Bombs);
                        println!("You lost!");

                        account.balance -= bet as f32;
                        break;
                    },
                    State::NextStep => {
                        self.current_step += 1;
                    }
                }
            }
            CommandLine::clear_terminal();
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
            State::WonGame(
                Burmalda::coefficient_for_step(self.current_step)
            )
        } else if self.bombs.get(self.current_step) == input {
            State::LostGame
        } else {
            State::NextStep
        }
    }

    fn ask_for_bet(account: &Account) -> f32 {
        loop {
            let bet = Bet::ask();
            if bet.0 > account.balance {
                println!("You don't have enough money!");
            } else {
                return bet.0;
            }
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
            print!("{}", format!("{:4.1}", Burmalda::coefficient_for_step(x)));
        }
        println!();
    }

}

impl Burmalda {

    fn element_to_print(&self, x: usize, y: usize) -> &str {
        if self.steps[x] == y { "X" } else { " " }
    }

    fn element_to_print_with_bombs(&self, x: usize, y: usize) -> &str {
        if self.bombs.get(x) == y { "*" } else if self.steps[x] == y { "X" } else { " " }
    }

}

impl Burmalda {

    fn coefficient_for_step(x: usize) -> f32 {
        if x == 0 { 1.0 } else { x as f32 * 1.25 }
    }

}