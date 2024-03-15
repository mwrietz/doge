use rand::Rng;

struct Game {
    bank: f64,
    wallet_init: f64,
    wallet_previous: f64,
    wallet: f64,
    bet_init: f64,
    bet_previous: f64,
    bet: f64,
    odds: f64,
    stop_loss_factor: f64,
    stop_win_factor: f64,
    stop_status: u32,
}

impl Game {
    fn one_round(&mut self) -> bool {
        let mut status = true;
        let mut rng = rand::thread_rng();
        let roll: f64 = rng.gen();

        self.wallet_previous = self.wallet;
        self.bet_previous = self.bet;
        if roll > (1.0 / self.odds) {
            self.wallet -= self.bet;
            self.bet *= 1.5;
            status = false;
        } else {
            self.wallet += self.bet * self.odds;
            self.bet = self.bet_init;
        }

        status
    }

    fn multi_round(&mut self, num_rounds: u32) {
        let mut win_status: bool = true;
        let mut status: String = "win".to_string();

        for i in 0..num_rounds {
            win_status = self.one_round();
            if win_status {
                status = "win".to_string();
            } else {
                status = "lose".to_string();
            }

            self.display_oneround_results(i, status);

            // determine if game should stop
            if self.stop() {
                println!("stop status: {}", self.stop_status);
                return;
            }
        }
    }

    fn stop(&mut self) -> bool {
        let mut stop = false;
        // bust
        if self.bet > self.wallet {
            stop = true;
            self.stop_status = 0;
            self.wallet = 0.0;
        }

        // stop loss
        if self.wallet < self.stop_loss_factor * self.wallet_init {
            stop = true;
            self.stop_status = 1;
        }

        // stop win
        if self.wallet > self.stop_win_factor * self.wallet_init {
            stop = true;
            self.stop_status = 2;
            self.bank += self.wallet - self.wallet_init;
            self.wallet = self.wallet_init;
        }

        stop
    }

    fn display_oneround_results(&self, i: u32, status: String) {
        println!(
            "i: {:4}  wallet: {:6.2}  bet: {:6.4}  status: {:6}  wallet: {:6.2}",
            i, self.wallet_previous, self.bet_previous, status, self.wallet
        );
    }

    fn display_multiround_results(&self) {
        println!(
            "wallet: {:8.2}, bank: {:8.2}, wallet+bank: {:8.2}",
            self.wallet,
            self.bank,
            self.wallet + self.bank
        );
    }
}

fn main() {
    let bet_factor = 10000.0;

    let mut game: Game = Game {
        bank: 0.0,
        wallet_init: 225.0,
        wallet_previous: 225.0,
        wallet: 225.0,
        bet_init: 225.0 / bet_factor,
        bet_previous: 225.0 / bet_factor,
        bet: 225.0 / bet_factor,
        odds: 3.0,
        stop_loss_factor: 0.8,
        stop_win_factor: 1.028,
        stop_status: 2,
    };
    for _i in 0..10 {
        game.multi_round(150);
        game.display_multiround_results();
    }
}
