use std::io;

use rand::Rng;

use crate::bust::is_bust;

pub fn round(bet: i32, rounds: i32) {
    for i in 1..=rounds {
        'gameloop: loop {
            // initialise all the variables
            let stdin = io::stdin();
            let mut rng = rand::thread_rng();
            let mut choice_string = String::new();
            let mut player_cards: Vec<i32> = vec![];
            let mut computer_cards: Vec<i32> = vec![];
            let mut player_sum = 0;
            let mut computer_sum = 0;
            if rounds != 1 {
                println!("Round {}", i);
            }
            // player draws first two cards
            let mut random_card = rng.gen_range(1..=11);
            println!("Your first card: {}", random_card);
            player_cards.push(random_card);
            random_card = rng.gen_range(1..=11);

            println!("Your second card: {}", random_card);
            player_cards.push(random_card);

            // print player hand
            println!("Your hand is now:");
            for card in &player_cards {
                print!("{}    ", card);
            }
            println!("");

            // check if player busted
            if is_bust(&player_cards) {
                println!("Bust! You lose!");
                break 'gameloop;
            }

            // reading user input from stdin and hitting or standing based on the input
            println!("Hit or stand? (1 to hit, 2 to stand)");
            stdin
                .read_line(&mut choice_string)
                .expect("Failed to read user input into choice variable");

            let choice: i32 = choice_string
                .trim()
                .parse()
                .expect("Invalid input, please input an number!");

            match choice {
                1 => {
                    random_card = rng.gen_range(1..=11);
                    println!("You have chosen to hit and got {}", random_card);
                    player_cards.push(random_card);
                }
                2 => println!("You have chosen to stand and will not draw a card"),
                _ => println!("You have chosen to stand and will not draw a card"),
            }

            // print player hand
            println!("Your hand is now:");
            for card in &player_cards {
                print!("{}    ", card);
            }
            println!("");

            // check if player busted
            if is_bust(&player_cards) {
                println!("Bust! You lose!");
            }

            // computer's turn
            println!("");
            println!("It is now the computer's turn");

            // computer's first two cards
            random_card = rng.gen_range(1..=11);
            computer_cards.push(random_card);
            random_card = rng.gen_range(1..=11);
            computer_cards.push(random_card);

            // check if computer busted
            if is_bust(&computer_cards) {
                println!("The computer busted! You win! The computer had ");
                for card in &computer_cards {
                    print!("{}    ", card)
                }
                break 'gameloop;
            }

            // computer randomly choses to hit or stand
            let choice = rng.gen_range(1..=2);
            if choice == 1 {
                random_card = rng.gen_range(1..=11);
                println!("The computer has chosen to hit and draws a card");
                computer_cards.push(random_card);
            } else {
                println!("The computer has chosen to stand and will not draw a card");
            }

            // check if compuer busted
            if is_bust(&computer_cards) {
                println!("The computer busted! You win! The computer had ");
                for card in &computer_cards {
                    print!("{}    ", card)
                }
                println!("");
                break 'gameloop;
            }

            // final comparison between player total card value and computer total card value
            println!("");
            println!("Showdown!");

            // Summing up the card values
            for card in &player_cards {
                player_sum += card;
            }

            for card in &computer_cards {
                computer_sum += card;
            }

            // comparing player_sum and computer_sum
            match player_sum {
                // if is is a tie
                n if n == computer_sum => {
                    println!("It is a tie! The computer had ");
                    for card in &computer_cards {
                        print!("{}    ", card)
                    }
                    println!("");

                    if bet != 0 {
                        println!("You neither lost or made any money")
                    }
                }
                // if you win
                n if n > computer_sum => {
                    println!("You won! The computer had ");
                    for card in &computer_cards {
                        print!("{}    ", card)
                    }
                    println!("");

                    if bet != 0 {
                        println!("You made ${}!", bet * 3);
                    }
                }
                // if you lose
                _ => {
                    println!("You lost! The computer had ");
                    for card in &computer_cards {
                        print!("{}    ", card)
                    }
                    println!("");

                    if bet != 0 {
                        println!("You lost ${}", bet);
                    }
                }
            }
            // graceful cleanup for hygiene
            choice_string.clear();
        }
    }
}
