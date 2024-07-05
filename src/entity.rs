#[derive(Debug)]
pub struct DocComment {
    pub comment: String
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub members: Vec<String>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: String,
    pub params: Vec<String>,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}
