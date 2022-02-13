pub struct Parser<'a> {
    tokens: &'a Vec<&'a str>,
    pos: std::cell::Cell<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<&str>) -> Self {
        Self {
            tokens: tokens,
            pos: std::cell::Cell::new(0),
        }
    }
    pub fn build(&self) -> Option<Box<crate::ast::ExprNode>> {
        self.build_expr()
    }
}

impl<'a> Parser<'a> {
    fn peek_token(&self) -> &str {
        if self.pos.get() < self.tokens.len() {
            return self.tokens[self.pos.get()];
        }
        ""
    }
    fn next_token(&self) {
        if self.pos.get() < self.tokens.len() {
            self.pos.set(self.pos.get() + 1);
        }
    }
}

impl<'a> Parser<'a> {
    fn build_expr(&self) -> Option<Box<crate::ast::ExprNode>> {
        let x = self.build_mul();
        match self.peek_token() {
            "+" => {
                self.next_token();
                let y = self.build_mul();

                return Some(Box::new(crate::ast::ExprNode {
                    value: "+",
                    left: x,
                    right: y,
                }));
            }
            "-" => {
                self.next_token();
                let y = self.build_mul();

                return Some(Box::new(crate::ast::ExprNode {
                    value: "-",
                    left: x,
                    right: y,
                }));
            }
            _ => {
                return x;
            }
        }
    }

    fn build_mul(&self) -> Option<Box<crate::ast::ExprNode>> {
        let x = self.build_primary();
        match self.peek_token() {
            "*" => {
                self.next_token();
                let y = self.build_primary();

                return Some(Box::new(crate::ast::ExprNode {
                    value: "*",
                    left: x,
                    right: y,
                }));
            }
            "/" => {
                self.next_token();
                let y = self.build_primary();

                return Some(Box::new(crate::ast::ExprNode {
                    value: "/",
                    left: x,
                    right: y,
                }));
            }
            _ => {
                return x;
            }
        }
    }
    fn build_primary(&self) -> Option<Box<crate::ast::ExprNode>> {
        let tok = self.peek_token();
        match tok {
            "(" => {
                self.next_token();
                let x = self.build_expr();
                self.next_token();
                return x;
            }
            _ => {
                self.next_token();
                return Some(Box::new(crate::ast::ExprNode {
                    value: tok,
                    left: None,
                    right: None,
                }));
            }
        }
    }
}
