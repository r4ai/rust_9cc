mod codegen;
mod parse;
mod result;
mod tokenize;

use codegen::gen;
use parse::program;
use tokenize::tokenize;

pub fn cli(args: Vec<String>) -> String {
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let mut tokens = tokenize(args[1].to_string()).unwrap();
    let mut result = String::new();

    // アセンブリの前半部分
    result.push_str(".intel_syntax noprefix\n");
    result.push_str(".globl main\n");
    result.push_str("main:\n");

    // 変数26個分の領域を確保する
    result.push_str("  push rbp\n");
    result.push_str("  mov rbp, rsp\n");
    result.push_str("  sub rsp, 208\n");

    let code = program(&mut tokens);
    for node in code {
        let asm_code = gen(&node);
        result.push_str(&asm_code);
        result.push_str("  pop rax\n");
    }

    result.push_str("  mov rsp, rbp\n");
    result.push_str("  pop rbp\n");
    result.push_str("  ret\n");
    return result;
}
