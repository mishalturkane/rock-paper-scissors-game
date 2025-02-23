use std::io;
extern crate rand;
use rand::Rng;
use std::process::Command;

fn main() {
    loop {
        clear_screen();
        println!("\n********* Welcome to Rock-Paper-Scissors *********");
        println!("You have 1 special 'Power Move' that beats everything! Use it wisely. ⚡");
        println!("Select an option: \n 1 -> Play Game \n 0 -> Exit");
        println!("Enter a number:");

        let choice = get_user_choice();

        if choice == 0 {
            println!("Exiting the game. Thanks for playing! 🎮");
            break;
        }

        play_with_power_move();

        println!("\nDo you want to play again? (y/n)");
        let mut replay = String::new();
        io::stdin().read_line(&mut replay).expect("Failed to read input");

        if replay.trim().to_lowercase() != "y" {
            println!("Thanks for playing! 🎮");
            break;
        }
    }
}

// Game with Power Move mode
fn play_with_power_move() {
    let mut user_score = 0;
    let mut computer_score = 0;
    let mut user_power_used = false;
    let mut computer_power_used = false;

    loop {
        clear_screen();
        println!("\n***** Rock-Paper-Scissors with Power Move *****");
        println!("You have 1 Power Move! Use it carefully. ⚡");
        println!("Select your move: \n 1 -> Rock \n 2 -> Paper \n 3 -> Scissors \n 4 -> POWER MOVE ⚡ (only once!) \n 0 -> Exit");
        println!("Enter a number:");

        let user_choice = get_user_choice();
        if user_choice == 0 {
            println!("Exiting game. Final Score - You: {}, Computer: {}", user_score, computer_score);
            break;
        }

        // If user selects Power Move but already used it, ask again
        if user_choice == 4 && user_power_used {
            println!("⚠️ You have already used your Power Move! Choose another option.");
            press_enter_to_continue();
            continue;
        }

        // Random computer choice (1-3 or 4 if power move isn't used yet)
        let mut rng = rand::thread_rng();
        let computer_choice = if !computer_power_used && rng.gen_range(0..=4) == 4 {
            4  // Computer uses Power Move
        } else {
            rng.gen_range(1..=3) // Normal move
        };

        // Prevent computer from using Power Move again
        if computer_choice == 4 {
            computer_power_used = true;
        }

        let result = determine_winner_with_power(user_choice, computer_choice, &mut user_power_used);
        
        if result == "You win!" {
            user_score += 1;
        } else if result == "Computer wins!" {
            computer_score += 1;
        }

        println!("Score: You {} - {} Computer", user_score, computer_score);
        press_enter_to_continue();
    }
}

// Determines winner, considering Power Move
fn determine_winner_with_power(user: i32, computer: i32, user_power_used: &mut bool) -> &'static str {
    match user {
        1 => println!("You chose: 🪨 Rock"),
        2 => println!("You chose: 📄 Paper"),
        3 => println!("You chose: ✂️ Scissors"),
        4 => {
            println!("⚡ You used POWER MOVE! ⚡");
            *user_power_used = true;
        },
        _ => {}
    }

    match computer {
        1 => println!("Computer chose: 🪨 Rock"),
        2 => println!("Computer chose: 📄 Paper"),
        3 => println!("Computer chose: ✂️ Scissors"),
        4 => println!("💀 Computer used POWER MOVE! 💀"),
        _ => {}
    }

    if user == 4 && computer != 4 {
        return "You win!";
    } else if computer == 4 && user != 4 {
        return "Computer wins!";
    } else if user == computer {
        return "It's a tie!";
    } else if (user == 1 && computer == 3) || (user == 2 && computer == 1) || (user == 3 && computer == 2) {
        return "You win!";
    } else {
        return "Computer wins!";
    }
}

// Function to clear screen
fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

// Function to handle user input
fn get_user_choice() -> i32 {
    loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read input");
        match temp.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input, please enter a valid number:");
            }
        }
    }
}

// Function to pause and ask user to press Enter before continuing
fn press_enter_to_continue() {
    println!("\nPress Enter to continue...");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");
}
