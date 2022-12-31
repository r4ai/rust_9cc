use rust_9cc::cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = cli(args);
    println!("{}", result);
}
