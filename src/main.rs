use std::io;
use rand::Rng;
fn main() {
    println!("********* Welcome to the game *********");
    println!("Select the input: \n 1->rock \n 2->paper \n 3->scissors");
    println!("Enter a number:");

   let number: i32 = {
       let mut temp = String::new();
       io::stdin().read_line(&mut temp).expect("Failed to read input");
       temp.trim().parse().expect("Invalid input, please enter a number")
   };

   match number {
       1 => println!("you chooses: rock"),
       2 => println!("you chooses: paper"),
       3 => println!("you chooses: scissors"),
       _ => {}
   }


   let mut rng = rand::thread_rng(); // Create a random number generator
   let random_number: i32 = rng.gen_range(1..=3); // Generate a number between 1 and 100

   match random_number {
       1 => println!("computer chooses: rock"),
       2 => println!("computer chooses: paper"),
       3 => println!("computer chooses: scissors"),
       _ => {}
   }


       let mut rng2 = rand::thread_rng(); // Create a random number generator
       let value: i32 = rng2.gen_range(1..=3); // Generate a number between 1 and 100

       match value {
           1 => println!("value is: rock"),
           2 => println!("value is: paper"),
           3 => println!("value is: scissors"),
           _ => {}

   }

   if number == value{
       println!("you win!");
   }
   else if random_number == value {
       println!("computer win!");
   }
       else if number==value && random_number == value {
           println!("tie!");
       }
   else {
       print!("you both lose!");
   }

}
