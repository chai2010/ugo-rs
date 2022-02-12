pub struct Compiler {
    next_id: i32,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { next_id: 0 }
    }

    pub fn gen_llir(&mut self, node: &crate::ast::ExprNode) -> String {
        let mut result = String::new();
        result.push_str("define i32 @main() {\n");
        let v = self.gen_value(&mut result, node);
        result.push_str(&format!("\tret i32 {}\n", v));
        result.push_str("}\n");
        result
    }
}

impl Compiler {
    fn gen_id(&mut self) -> String {
        let s = format!("%t{}", self.next_id);
        self.next_id += 1;
        s
    }

    fn gen_value(&mut self, output: &mut String, node: &crate::ast::ExprNode) -> String {
        let id = self.gen_id();
        match &node.value {
            &"+" => {
                let x = self.gen_value(output, node.left.as_ref().unwrap());
                let y = self.gen_value(output, node.right.as_ref().unwrap());
                output.push_str(&format!("\t{} = add i32 {}, {}\n", id, x, y));
                return id;
            }
            &"-" => {
                let x = self.gen_value(output, node.left.as_ref().unwrap());
                let y = self.gen_value(output, node.right.as_ref().unwrap());
                output.push_str(&format!("\t{} = sub i32 {}, {}\n", id, x, y));
                return id;
            }
            &"*" => {
                let x = self.gen_value(output, node.left.as_ref().unwrap());
                let y = self.gen_value(output, node.right.as_ref().unwrap());
                output.push_str(&format!("\t{} = mul i32 {}, {}\n", id, x, y));
                return id;
            }
            &"/" => {
                let x = self.gen_value(output, node.left.as_ref().unwrap());
                let y = self.gen_value(output, node.right.as_ref().unwrap());
                output.push_str(&format!("\t{} = sdiv i32 {}, {}\n", id, x, y));
                return id;
            }
            _ => {
                output.push_str(&format!("\t{} = add i32 0, {}\n", id, node.value));
                return id;
            }
        }
    }
}
