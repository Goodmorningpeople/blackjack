use clap::ArgMatches;

use crate::gameloop::round;

pub fn match_play(play_args: Option<&ArgMatches>) {
    let mut bet = 0;
    let mut rounds = 1;
    match play_args {
        Some(args) => {
            if let Some(bet_option) = args.get_one::<i32>("bet-option") {
                bet = *bet_option;
            } 
            if let Some(round_option) = args.get_one::<i32>("round-option") {
                rounds = *round_option;
            } 
            round(bet / rounds, rounds);
        }
        None => {}
    }
}