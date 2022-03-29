pub enum Guess {
    Position(usize),
    Quit
}

impl Guess {

    pub fn new() -> Guess {
        let mut input = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut input) {
            if let Ok(input) = input.trim().parse::<usize>() {
                Guess::Position(input)
            } else if input == "q\n" {
                Guess::Quit
            } else {
                println!("Invalid input!: '{}'", input);
                Guess::new()
            }
        } else {
            Guess::Quit
        }
    }

}
