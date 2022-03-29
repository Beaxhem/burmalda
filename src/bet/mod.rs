use crate::Account;

pub struct Bet(pub f32);

impl Bet {

    fn ask() -> Bet {
        println!("How much do you want to bet?");
        let mut input = String::new();
        if let (Ok(_), Ok(input)) = (std::io::stdin().read_line(&mut input), input.trim().parse::<f32>()) {
            Bet(input)            
        } else {
            Bet::ask()
        }
    }

    pub fn ask_for_bet(account: &Account) -> f32 {
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