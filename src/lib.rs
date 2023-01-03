mod codegen;
mod parse;
mod result;
mod tokenize;

use codegen::gen;
use parse::expr;
use tokenize::tokenize;

pub fn cli(args: Vec<String>) -> String {
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let mut tokens = tokenize(args[1].to_string()).unwrap();
    let mut result = String::new();

    result.push_str(".intel_syntax noprefix\n");
    result.push_str(".globl main\n");
    result.push_str("main:\n");

    let node = expr(&mut tokens);
    // dbg!(&node);

    let asm_code = gen(&node);
    result.push_str(asm_code.as_str());

    result.push_str("  pop rax\n");
    result.push_str("  ret\n");
    return result;
}
