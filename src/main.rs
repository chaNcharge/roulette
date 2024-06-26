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

    let mut dealer_revives = 0;
    let mut player_revives = 0;

    while dealer_revives >= 0 && player_revives >= 0 {
        let num_shells = rand::thread_rng().gen_range(3..9); // At least 3 shells, up to 8 max
        let mut shells: Vec<i32> = vec![];
        // TODO: Figure out way to have at least one guaranteed 0 and 1 somewhere in gun
        for _ in 1..=num_shells {
            let shell: i32 = rand::thread_rng().gen_range(0..=1);
            shells.push(shell);
        }
        println!("Shells this round:");
        for shell in &shells {
            print!("{shell} ");
        }
        println!();

        // Load gun with shells in random order (aka shuffle the shells)
        shells.shuffle(&mut rand::thread_rng());
        println!("Gun is loaded in this order:"); // debugging
        for i in &shells {
            print!("{i} ");
        }
        println!();

        dealer_revives -= 1;
    }
    println!("Game over");
}
