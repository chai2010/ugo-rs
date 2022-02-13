// rustup doc

#![allow(unused)]

mod ast;
mod compiler;
mod lex;
mod parser;

fn main() {
    let ret = run("1+2*(3+4)");
    println!("{}", ret);
}

fn run(code: &str) -> i32 {
    let tokens = crate::lex::lex(code);

    let parser = crate::parser::Parser::new(&tokens);
    let node = parser.build();

    compile(node.as_ref().unwrap());

    let status = std::process::Command::new("./a.out").status().unwrap();

    match status.code() {
        Some(code) => code,
        None => -1,
    }
}

fn compile(node: &crate::ast::ExprNode) {
    let mut c = crate::compiler::Compiler::new();
    let output = c.gen_llir(node);

    std::fs::write("a.out.ll", output).unwrap();

    std::process::Command::new("clang")
        .arg("-Wno-override-module")
        .arg("-o")
        .arg("a.out")
        .arg("a.out.ll")
        .output()
        .unwrap();
}
