pub mod cardutils;
pub mod basicstrategy;
pub mod gamelogic;

use gamelogic::*;
use basicstrategy::*;
//use cardutils::*;
fn main() {
    //let mut thread_pool = vec![];
    let bankroll = 1000000000000000;
    let num_games = 10_000_000;
    //println!("Simulating {} games of blackjack across 12 threads, 6 cores", num_games);
    //for _ in 0..12 {
    //    let rules = GameRules::new(6, 0.85, false, true, None, true, false, false);
    //    let count = Count::new(100, 0.0);
    //    let mut game = Game::new(rules, count, bankroll, num_games / 12);
    //    thread_pool.push(std::thread::spawn(move || {
    //        game.play();
    //    }));
    //}
    //
    //for thread in thread_pool {
    //    thread.join().unwrap();
    //}
    let rules = GameRules::new(6, 47, false, true, None, true, false, false);
    let count = Count::new(100, 0.0);
    let game = Game::new(rules, count, bankroll, num_games);
    BasicStrategyLUT::calculate_cache(&game);
    println!("Simulating {} games of blackjack", num_games);
    let game_results = game.play();
    println!("Amount Won: {:?}", game_results.amount_won);
    println!("Amount Lost: {:?}", game_results.amount_lost);
    println!("Bankroll: {:?}", game_results.bankroll);
}
