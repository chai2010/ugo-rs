// rustup doc

#![allow(unused)]

mod ast;
mod compiler;
mod lex;

fn main() {
    // 1+2*(3+4)
    let node = crate::ast::ExprNode {
        value: "+",
        left: Some(Box::new(crate::ast::ExprNode {
            value: "1",
            left: None,
            right: None,
        })),
        right: Some(Box::new(crate::ast::ExprNode {
            value: "*",
            left: Some(Box::new(crate::ast::ExprNode {
                value: "2",
                left: None,
                right: None,
            })),
            right: Some(Box::new(crate::ast::ExprNode {
                value: "+",
                left: Some(Box::new(crate::ast::ExprNode {
                    value: "3",
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(crate::ast::ExprNode {
                    value: "4",
                    left: None,
                    right: None,
                })),
            })),
        })),
    };

    println!("{}", run(&node));
}

fn run(node: &crate::ast::ExprNode) -> i32 {
    compile(node);

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
