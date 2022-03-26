use rand::Rng;
use std::io::*;

fn print_with_spaces(word: String) {
    print!("\t");
    for i in 0..word.len() {
        print!("{} ", word.chars().nth(i).unwrap());
    }
    print!("\n");
}

fn hide_word(word: String) -> String {
    let mut buffer = String::with_capacity(word.len());

    for _i in 0..word.len() {
        buffer.push('_');
    }

    return buffer;
}

// takes a char and compares it to every letter from the hidden string
fn reveal_letter(letter: char, reference: String, target: &mut String) {
    if reference.contains(letter) {
        for i in 0..reference.len() {
            if reference.chars().nth(i).unwrap() == letter {
                target.replace_range(i..(i+1), &letter.to_string());
            }
        }
    }
}

fn main() {
    let dict:  [&str; 5] = ["monument", "potatoe", "strange", "orange", "meadow"];
    let word = dict[rand::thread_rng().gen_range(0..dict.len())].to_string();
    let mut output = hide_word(word.clone());
    let mut input = String::new();
    let mut attempts: u8 = 10;
    
    while attempts > 0 {
        print_with_spaces(output.clone());
        
        print!(">> ");
        // the next line flushes the buffer before the input comes
        stdout().flush().expect("error flushing");
        
        stdin().read_line(&mut input).expect("error reading");
        input = String::from(input.trim());
        
        reveal_letter(input.chars().next().unwrap(), word.clone(), &mut output);
        
        if output == word { break; }
        println!("You have {attempts} chances left.");
        
        input.clear();
        attempts -= 1;
    }

    if attempts > 0 {
        println!("You guessed it!");
    } else {
        println!("You lost!");
    }
    
    print!("The word was: ");
    print_with_spaces(word.clone());
}