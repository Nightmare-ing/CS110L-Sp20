use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let buf = fs::read_to_string(filename).expect("Failed to open file!");
    let num_chars = buf.len();
    let num_lns = buf.split("\n").count() - 1;
    let num_words = buf.split_whitespace().count();
    println!("{} {} {}", num_lns, num_words, num_chars);
}
