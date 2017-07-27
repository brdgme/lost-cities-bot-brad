extern crate lost_cities_bot_brad;
extern crate lost_cities;
extern crate brdgme_game;

use lost_cities_bot_brad::Brad;
use brdgme_game::bot::Fuzzer;

use std::io::stdout;

fn main() {
    let mut f = Fuzzer::new(Brad {});
    f.fuzz(&mut stdout());
}