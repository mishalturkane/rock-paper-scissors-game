use std::io;
extern crate rand;
use rand::Rng;

fn main() {
    let mut user_score = 0;
    let mut computer_score = 0;

    loop {
        println!("\n********* Welcome to the game *********");
        println!("Select the input: \n 1 -> Rock \n 2 -> Paper \n 3 -> Scissors \n 0 -> Exit");
        println!("Enter a number:");

        let number: i32 = {
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Failed to read input");
            match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, please enter a number.");
                    continue;
                }
            }
        };

        if number == 0 {
            println!("Exiting game. Final Score - You: {}, Computer: {}", user_score, computer_score);
            break;
        }

        if number < 1 || number > 3 {
            println!("Invalid choice, please enter a number between 1 and 3.");
            continue;
        }

        match number {
            1 => println!("You chose: Rock"),
            2 => println!("You chose: Paper"),
            3 => println!("You chose: Scissors"),
            _ => {}
        }

        let mut rng = rand::thread_rng();
        let computer_choice: i32 = rng.gen_range(1..=3);

        match computer_choice {
            1 => println!("Computer chose: Rock"),
            2 => println!("Computer chose: Paper"),
            3 => println!("Computer chose: Scissors"),
            _ => {}
        }

        if number == computer_choice {
            println!("It's a tie!");
        } else if (number == 1 && computer_choice == 3) || (number == 2 && computer_choice == 1) || (number == 3 && computer_choice == 2) {
            println!("You win!");
            user_score += 1;
        } else {
            println!("Computer wins!");
            computer_score += 1;
        }

        println!("Current Score -> You: {}, Computer: {}", user_score, computer_score);
    }
}
