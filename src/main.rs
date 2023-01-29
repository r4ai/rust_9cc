use rust_9cc::cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let args = vec![" ".to_string(), "a=1;a+1;".to_string()];
    let result = cli(args);
    println!("{result}");
}
