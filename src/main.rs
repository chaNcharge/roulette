use std::io::stdin;
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    println!("Welcome to Russian Roulette... Enter your name.");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    println!("Hello, {}.", name);
    println!("I will load a shotgun with a random number of blanks and live ammo in a random order. 
Neither of us will know the order. We each get two revives. 
First to three deaths loses and dies permanently. Winner takes all.");
    println!("Let's play... This should be interesting...");

    let mut dealer_revives = 2;
    let mut player_revives = 2;

    let num_shells = rand::thread_rng().gen_range(3..9); // At least 3 shells, up to 8 max
    let mut shells: Vec<i32> = vec![];
    // TODO: Figure out way to have at least one guaranteed 0 and 1 somewhere in gun
    for _ in 1..=num_shells {
        let shell: i32 = rand::thread_rng().gen_range(0..=1);
        shells.push(shell);
    }
    // All 0 or all 1 shell check
    if shells.iter().all(|&x| x == 0) {
        shells[0] = 1;
    } else if shells.iter().all(|&x| x == 1) {
        shells[0] = 0;
    }

    println!("Shells this round:");
    for shell in &shells {
        print!("{shell} ");
    }
    println!();
    
    // Load gun with shells in random order (aka shuffle the shells)
    shells.shuffle(&mut rand::thread_rng());
    println!("Gun is loaded in this order (note this is reverse as pop pops last element):"); // debugging
    for i in &shells {
        print!("{i} ");
    }
    println!();

    while dealer_revives >= 0 && player_revives >= 0 && !shells.is_empty() {
        let mut player_choice_input: String = String::new();
        println!("Enter 0 to shoot yourself, or 1 to shoot the dealer.");
        stdin().read_line(&mut player_choice_input).unwrap();
        let player_choice: i32 = player_choice_input.trim().parse().unwrap();
        println!("You chose: {}", player_choice);
        let current_shot = shells.pop().unwrap();
        if current_shot == 0 {
            println!("Blank round");
            if player_choice == 0 {
                println!("You survived"); // Player goes again
            } else {
                println!("Dealer survived"); // Players turn ends, dealer goes
            }
        } else {
            println!("Live round");
            if player_choice == 0 {
                println!("You died");
                player_revives -= 1;
            } else {
                println!("Dealer died");
                dealer_revives -= 1;
            }
        }
    }
    println!("Game over");
}
