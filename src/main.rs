use blackjack::play::match_play;
use clap::{command, value_parser, Arg, Command};

fn main() {
    let match_result = command!()
        .about("Blackjack cli tool that allows you to place bets")
        .subcommand(Command::new("play")
            .about("play [options]: command to start a game, options:
bet [amount]: bet an amount of money, the amount of money you win/lose will be shown at the end of a round 
round [number-of-rounds]: set the amount of rounds you would like to play
")
            .arg(
                Arg::new("bet-option")
                    .short('b')
                    .long("bet")
                    .value_parser(value_parser!(i32))
            )
            .arg(
                Arg::new("round-option")
                    .short('r')
                    .long("round")
                    .value_parser(value_parser!(i32))
            )
        ).get_matches();
    
    let play_args = match_result.subcommand_matches("play");
    match_play(play_args);
}
