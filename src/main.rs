// rustup doc

// echo 123 | cargo run
// ./a.out
// echo $?

fn main() {
	let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    compile(buffer.as_ref());
}

fn compile(code: &str) {
    let output = format!(
        r"
define i32 @main() {{
	ret i32 {}
}}
",
        code
    );

    std::fs::write("a.out.ll", output).unwrap();

	std::process::Command::new("clang")
		.arg("-Wno-override-module")
		.arg("-o")
		.arg("a.out")
		.arg("a.out.ll")
		.output()
		.unwrap();
}
