pub fn lex(code: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    loop {
        let mut found_idx = None;
        for i in pos..code.len() {
            match &code[i..][..1] {
                "+" | "-" | "*" | "/" | "(" | ")" => {
                    found_idx = Some(i);
                    break;
                }
                _ => {}
            }
        }
        if let Some(i) = found_idx {
            if i > pos {
                tokens.push(&code[pos..i]);
            }
            tokens.push(&code[i..][..1]);
            pos = i + 1;
            continue;
        }

        if pos < code.len() {
            tokens.push(&code[pos..]);
        }
        return tokens;
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_works() {
        assert_eq!(lex("1"), vec!["1"]);
        assert_eq!(lex("1+22*333"), vec!["1", "+", "22", "*", "333"]);
        assert_eq!(
            lex("1+22*(3+4)"),
            vec!["1", "+", "22", "*", "(", "3", "+", "4", ")"]
        );
    }
}
