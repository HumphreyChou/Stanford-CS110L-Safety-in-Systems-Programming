// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    // Your code here! :)
    let mut incorrect_cnt = 0;
    let mut set: HashSet<usize> = HashSet::new();
    let mut guesses = String::from("");
    while incorrect_cnt < NUM_INCORRECT_GUESSES && set.len() < secret_word_chars.len() {
        print!("The word so far is: ");
        for i in 0..secret_word_chars.len() {
            if set.contains(&i) {
                print!("{}", secret_word_chars[i]);
            } else {
                print!("-");
            }
        }
        print!("\n");
        println!("You have guessed: {}", guesses);

        print!("Please enter a letter: ");
        io::stdout().flush().expect("error flushing stdout");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error reading line");
        let c = guess.as_bytes()[0];
        let c = c as char;
        guesses.push(c);
        let mut i = 0;
        while i < secret_word_chars.len() {
            if c == secret_word_chars[i] {
                set.insert(i);
                break;
            }
            i += 1;
        }
        if i == secret_word_chars.len() {
            println!("Wrong guess");
            incorrect_cnt += 1;
        }
        println!("");
    }
    if incorrect_cnt == NUM_INCORRECT_GUESSES {
        println!("Sorry, you run out of guesses");
    } else {
        println!("You win!");
    }
}
