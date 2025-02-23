use std::io;
extern crate rand;
use rand::Rng;
use std::process::Command;

fn main() {
    let mut user_score = 0;
    let mut computer_score = 0;
    let mut rounds_played = 0;
    let mut user_streak = 0;
    let mut computer_streak = 0;

    loop {
        clear_screen();
        println!("\n********* Welcome to the game *********");
        println!("Select the input: \n 1 -> Rock \n 2 -> Paper \n 3 -> Scissors \n 0 -> Exit");
        println!("Enter a number:");

        let number = get_user_choice();
        
        if number == 0 {
            println!("Exiting game. Final Score - You: {}, Computer: {}", user_score, computer_score);
            println!("Total Rounds Played: {}", rounds_played);
            break;
        }

        rounds_played += 1;

        match number {
            1 => println!("You chose: 🪨 Rock"),
            2 => println!("You chose: 📄 Paper"),
            3 => println!("You chose: ✂️ Scissors"),
            _ => {}
        }

        let mut rng = rand::thread_rng();
        let computer_choice: i32 = rng.gen_range(1..=3);

        match computer_choice {
            1 => println!("Computer chose: 🪨 Rock"),
            2 => println!("Computer chose: 📄 Paper"),
            3 => println!("Computer chose: ✂️ Scissors"),
            _ => {}
        }

        let result = determine_winner(number, computer_choice);

        match result {
            "You win!" => {
                user_score += 1;
                user_streak += 1;
                computer_streak = 0;
                println!("🔥 You win!");
            }
            "Computer wins!" => {
                computer_score += 1;
                computer_streak += 1;
                user_streak = 0;
                println!("💀 Computer wins!");
            }
            "It's a tie!" => {
                println!("⚖️ It's a tie!");
                user_streak = 0;
                computer_streak = 0;
            }
            _ => {}
        }

        println!("Current Score -> You: {}, Computer: {}", user_score, computer_score);
        println!("Total Rounds Played: {}", rounds_played);

        // Display Win Streak
        if user_streak > 1 {
            println!("🔥 You are on a {}-win streak!", user_streak);
        }
        if computer_streak > 1 {
            println!("💀 Computer is on a {}-win streak!", computer_streak);
        }

        println!("\nPress Enter to continue...");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read input");
    }

    println!("\nDo you want to play again? (y/n)");
    let mut replay = String::new();
    io::stdin().read_line(&mut replay).expect("Failed to read input");

    if replay.trim().to_lowercase() == "y" {
        main();
    } else {
        println!("Thanks for playing! 🎮");
    }
}

// Function to determine the winner
fn determine_winner(user: i32, computer: i32) -> &'static str {
    if user == computer {
        return "It's a tie!";
    } else if (user == 1 && computer == 3) || (user == 2 && computer == 1) || (user == 3 && computer == 2) {
        return "You win!";
    } else {
        return "Computer wins!";
    }
}

// Function to clear screen for better UI
fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

// Function to handle user input validation
fn get_user_choice() -> i32 {
    loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read input");
        match temp.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input, please enter a valid number (1, 2, 3, or 0 to exit):");
            }
        }
    }
}
