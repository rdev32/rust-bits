use rand::Rng;
use std::io;

fn main() 
{
    let random_number: i16 = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();
    let mut choise: i16; 

    println!("Guess a number between 1 and 100!");

    let mut attempts: i16 = 5;
    while attempts > 0 
    {   
        io::stdin().read_line(&mut guess).expect("Error reading");
        choise = guess.trim().parse::<i16>().unwrap();
        
        if random_number == choise 
        {
            println!("Nice, you guessed it!");
            break;
        }

        if choise < random_number 
        {
            println!("Hint: {} is below the number", choise);
        }

        if choise > random_number 
        {
            println!("Hint: {} is above the number", choise);
        }

        guess.clear();
        attempts -= 1;
    }

    print!("Game Over: ");

    if attempts == 0 
    {
        println!("You lost, the number was {}", random_number);
        return;
    }

    println!("You won!");
    return;
}
    
