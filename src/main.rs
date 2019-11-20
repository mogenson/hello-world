use rand::{thread_rng, Rng};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

/* this program brute force guesses the target string */
/* and prints the guesses along the way */

fn main() {
    let target = "Hello, world!";
    let mut current = String::new();
    let mut rng = thread_rng();

    for ch in target.chars() {
        loop {
            sleep(Duration::from_millis(10)); // slow down to see printing
            let guess: u8 = rng.gen_range(32, 128); // printable ascii range
            let guess = guess as char; // convert to char
            write!(stdout(), "\r{}{}", current, guess).unwrap(); // print
            stdout().flush().unwrap(); // flush
            if guess == ch {
                current.push(guess); // add to current string if correct
                break;
            }
        }
    }

    write!(stdout(), "\n").unwrap(); // print new line
}
