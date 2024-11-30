pub mod basicstrategy;
pub mod cardutils;
pub mod gamelogic;
use smallvec::{SmallVec, smallvec};
use std::thread::JoinHandle;
use std::time::Instant;

use gamelogic::*;
use basicstrategy::*;
//use cardutils::*;
fn main() {
    let mut thread_pool: SmallVec<[JoinHandle<()>; 11]> = smallvec![];
    let bankroll = 1000000000000000;
    let num_games = 100_000_000;
    let rules = GameRules::new(6, 47, false, true, None, false, false, false);
    BasicStrategyLUT::calculate_cache(&Game::new(rules, bankroll, num_games / 12));
    println!("Simulating {} games of blackjack across 12 threads, 6 cores", num_games);
    let timer = Instant::now();
    for _ in 0..11 {
        let rules = GameRules::new(6, 47, false, true, None, false, false, false);
        let game = Game::new(rules, bankroll, num_games / 12);
        thread_pool.push(std::thread::spawn(move || {
            game.play();
        }));
    }
    let rules = GameRules::new(6, 47, false, true, None, false, false, false);
    let game = Game::new(rules, bankroll, num_games / 12);

    game.play();
    for thread in thread_pool {
        thread.join().unwrap();
    }
    let elapsed = timer.elapsed();
    println!("Time taken: {} nanoseconds", elapsed.as_nanos());
    println!("Time taken: {} milliseconds", elapsed.as_millis());
    println!("Time taken: {} seconds", elapsed.as_secs());


    //let bankroll = 1000000000000000;
    //let num_games = 1000000000;
    //let rules = GameRules::new(6, 47, false, true, None, false, false, false);
    //let game = Game::new(rules, bankroll, num_games / 12);
    //let timer = Instant::now();
    //let game_results = game.play();
    //let elapsed = timer.elapsed();
    //
    //println!("Amount Won: {:?}", game_results.amount_won);
    //println!("Amount Lost: {:?}", game_results.amount_lost);
    //println!("Bankroll: {:?}", game_results.bankroll);
    //println!("Time taken: {} nanoseconds", elapsed.as_nanos());
    //println!("Time taken: {} milliseconds", elapsed.as_millis());
    //println!("Time taken: {} seconds", elapsed.as_secs());
}
