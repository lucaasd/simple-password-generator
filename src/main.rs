use std::io;

fn main() {
    println!("> This is a password generator :)");

    let mut length_str = String::new();

    println!("> Now, I need the length/size of your password.");
    io::stdin().read_line(& mut length_str).expect("Error reading line :(");

    let length = length_str.trim().parse().unwrap(); // Unwrap to make it simpler
    println!("Ok, received password length {}. Proceeding...", length);

    let mut password = String::new();

    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let uppercase_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let numbers = "1234567890".to_string();
    let special = "!@#$%¨&*()_+`{^}:?-=´[]~,.;/";
    let chars = alphabet + &uppercase_alphabet + &numbers + &special;

    for _ in 0..length {
        let alphabet_index = rand::random_range(0..chars.chars().count()) as usize;
        let letter = chars.chars().nth(alphabet_index);
        password.push(letter.unwrap());
    }

    println!("Here is your password: :)\n{}", password)
}
