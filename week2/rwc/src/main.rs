use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    
    let file = File::open(filename).unwrap_or_else(|_| panic!("fail to open {}", filename));

    let mut line_cnt = 0;
    let mut word_cnt = 0;
    let mut char_cnt = 0;

    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(str) => {
                line_cnt += 1;
                char_cnt += str.len();
                word_cnt += str.split(' ').collect::<Vec<&str>>().len();
            },
            Err(err) => panic!("fail to read at line {}", line_cnt)
        };
    }

    println!("line: {} word: {} char: {}", line_cnt, word_cnt, char_cnt);
}
