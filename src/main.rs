use std;
use rand::{Rng};

const NUMBER_OF_STEPS: usize = 8;
const NUMBER_OF_LINES: usize = 5;
struct Burmalda {
    current_step: usize,
    steps: [usize; NUMBER_OF_STEPS],
    bombs: [usize; NUMBER_OF_STEPS]
}

impl Burmalda {

    fn new() -> Burmalda {
        Burmalda { 
            current_step: 0,
            steps: [0; NUMBER_OF_STEPS],
            bombs: [0; NUMBER_OF_STEPS]
        }
    }

    fn play(&mut self) {
        self.generate_bombs();

        loop {
            self.print_game();
            if let Some(input) = Burmalda::guess() {
                if !self.check_input(input) {
                    break;
                }
            }
            Burmalda::clear_terminal();
        }
        Burmalda::clear_terminal();
        self.print_game();
        println!("You lost!");
    }

    fn guess() -> Option<usize> {
        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            return None;
        }

        if let Ok(input) = input.trim().parse::<usize>() {
            return Some(input)
        }
        
        return None;
    }

    fn check_input(&mut self, input: usize) -> bool {
        self.steps[self.current_step] = input;
        if self.bombs[self.current_step] == input {
            return false;
        }
        self.current_step += 1;
        true
    }

}

impl Burmalda {

    fn generate_bombs(&mut self) {
        for i in 0..NUMBER_OF_STEPS {
            self.bombs[i] = rand::thread_rng().gen_range(1..=NUMBER_OF_LINES+1);
        }
    }
    
}

impl Burmalda {

    fn print_game(&self) {
        let mut y = 1;
        for _ in 0..NUMBER_OF_LINES {
            Burmalda::print_line();
            print!("|");
            for x in 0..NUMBER_OF_STEPS {

                let to_print = if self.steps[x] == y { 
                    if x == self.current_step { "*" } else { "X" }
                } else { 
                    " " 
                };
                print!(" {} |", to_print);
            }
            print!("\n");
            y += 1;
        }
        Burmalda::print_line();
        print!("\n");
        Burmalda::print_coefficients();
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

fn main() {

    Burmalda::new().play()
    
}
