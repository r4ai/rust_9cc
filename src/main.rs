use rust_9cc::{cli, Node};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let args = vec![" ".to_string(), "3 + 7 - 2".to_string()];
    let result = cli(args);
    println!("{}", result);
}
