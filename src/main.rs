mod account;
mod bet;
mod burmalda;
mod cli;

use burmalda::Burmalda;
use account::Account;

struct GameSession {
    account: Account,
}

impl GameSession {

    fn new() -> GameSession {
        let account = Account { balance: 100.0 };
        GameSession { account }
    }

    fn play(&mut self) {
        loop {
            let mut game = Burmalda::new();
            game.play(&mut self.account);
            println!("Your balance is {}", self.account.balance);

            if self.account.balance <= 1.0 {
                break;
            }
        }
    }

}

fn main() {
    GameSession::new().play();
}
