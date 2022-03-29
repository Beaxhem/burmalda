mod bombs;
mod bet;
mod guess;
mod grid;

use crate::{Account, cli::CommandLine, burmalda::{bombs::Bombs, bet::Bet, guess::Guess, grid::Grid}};

pub const NUMBER_OF_STEPS: usize = 8;
pub const NUMBER_OF_LINES: usize = 5;

enum State {
    InvalidInput,
    LostGame,
    WonGame,
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
            match Guess::new() {
                Guess::Position(guess) => {
                    match self.check_input(guess) {
                        State::InvalidInput => { },
                        State::WonGame => {
                            CommandLine::clear_terminal();
                            self.print_game(PrintVariant::Bombs);
                            println!("You won!");
                            self.win_game(bet, account);
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
                },
                Guess::Quit => {
                    self.win_game(bet, account);
                    break;
                },
            }
            CommandLine::clear_terminal();
        }
    }

    fn check_input(&mut self, input: usize) -> State {
        if input < 1 || input > NUMBER_OF_LINES {
            return State::InvalidInput;
        }

        self.steps[self.current_step] = input;

        if self.current_step == NUMBER_OF_STEPS - 1 {
            State::WonGame
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

    fn win_game(&self, bet: f32, account: &mut Account) {
        let coefficient = Burmalda::coefficient_for_step(self.current_step);
        account.balance += bet as f32 * coefficient - bet; 
    }

}

enum PrintVariant {
    Normal,
    Bombs
}

impl Burmalda {

    fn print_game(&self, variant: PrintVariant) {
        Grid::print(NUMBER_OF_STEPS, NUMBER_OF_LINES, |x, y| {
            match variant {
                PrintVariant::Normal => self.element_to_print(x, y),
                PrintVariant::Bombs => self.element_to_print_with_bombs(x, y)
            }
        });    
        
        Burmalda::print_coefficients();
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

    fn element_to_print<'a>(&self, x: usize, y: usize) -> &'a str {
        if self.steps[x] == y { "X" } else { " " }
    }

    fn element_to_print_with_bombs<'a>(&self, x: usize, y: usize) -> &'a str {
        if self.bombs.get(x) == y { "*" } else if self.steps[x] == y { "X" } else { " " }
    }

}

impl Burmalda {

    fn coefficient_for_step(x: usize) -> f32 {
        if x == 0 { 1.0 } else { x as f32 * 1.25 }
    }

}