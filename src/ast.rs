#[derive(Debug)]
pub struct ExprNode<'a> {
    pub value: &'a str, // +, -, *, /, 123
    pub left: Option<Box<ExprNode<'a>>>,
    pub right: Option<Box<ExprNode<'a>>>,
}
