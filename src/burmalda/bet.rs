pub struct Bet(pub f32);

impl Bet {

    pub fn ask() -> Bet {
        println!("How much do you want to bet?");
        let mut input = String::new();
        if let (Ok(_), Ok(input)) = (std::io::stdin().read_line(&mut input), input.trim().parse::<f32>()) {
            Bet(input)            
        } else {
            Bet::ask()
        }
    }

}