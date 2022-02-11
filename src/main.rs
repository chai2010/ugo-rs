// rustup doc

// echo 1+3-2 | cargo run
// ./a.out
// echo $?

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    println!("{}", run(buffer.as_ref()));
}

fn run(code: &str) -> i32 {
    compile(code);

    let status = std::process::Command::new("./a.out").status().unwrap();

    match status.code() {
        Some(code) => code,
        None => -1,
    }
}

fn compile(code: &str) {
    let tokens = parse_tokens(code);
    let output = gen_asm(&tokens);

    std::fs::write("a.out.ll", output).unwrap();

    std::process::Command::new("clang")
        .arg("-Wno-override-module")
        .arg("-o")
        .arg("a.out")
        .arg("a.out.ll")
        .output()
        .unwrap();
}

fn gen_asm(tokens: &[&str]) -> String {
    let mut result = String::new();

    result.push_str("define i32 @main() {\n");

    let mut idx = 0;
    for (i, tok) in tokens.iter().enumerate() {
        if i == 0 {
            result.push_str(&format!("\t%t{} = add i32 0, {}\n", idx, tok));
            continue;
        }
        match tok {
            &"+" => {
                idx = idx + 1;
                result.push_str(&format!(
                    "\t%t{} = add i32 %t{}, {}\n",
                    idx,
                    idx - 1,
                    tokens[i + 1]
                ));
            }
            &"-" => {
                idx = idx + 1;
                result.push_str(&format!(
                    "\t%t{} = sub i32 %t{}, {}\n",
                    idx,
                    idx - 1,
                    tokens[i + 1]
                ));
            }
            _ => {}
        }
    }

    result.push_str(&format!("\tret i32 %t{}\n", idx));
    result.push_str("}\n");

    result
}

fn parse_tokens(code: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    loop {
        if let Some(i) = code[pos..].find('+') {
            tokens.push(&code[pos..][..i]);
            tokens.push(&code[pos..][i..][..1]);
            pos = pos + i + 1;
            continue;
        }
        if let Some(i) = code[pos..].find('-') {
            tokens.push(&code[pos..][..i]);
            tokens.push(&code[pos..][i..][..1]);
            pos = pos + i + 1;
            continue;
        }

        tokens.push(&code[pos..]);
        return tokens;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_works() {
        assert_eq!(run("1"), 1);
        assert_eq!(run("1+1"), 2);
        assert_eq!(run("1 + 3 - 2"), 2);
        assert_eq!(run("1+2+3+4"), 10);
    }
}
