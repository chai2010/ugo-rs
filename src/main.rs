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


	//, , , 

    //let output = format!(tmpl.as_ref(), code);
    //println!(output);
}

/*
use std::process::Command;

    Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")

*/

//define i32 @main() {
//    ret i32 %v
//}
//";
/*
func compile(code string) {
    output := fmt.Sprintf(tmpl, code)
    os.WriteFile("a.out.ll", []byte(output), 0666)
    exec.Command("clang", "-Wno-override-module", "-o", "a.out", "a.out.ll").Run()
}

const tmpl = `
define i32 @main() {
    ret i32 %v
}
`
*/
