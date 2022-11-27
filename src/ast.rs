pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    While(While),
    Assignment(Assignment),
}

#[derive(Debug)]
pub struct While {
    pub condition: u64,
    pub program: Program,
}

#[derive(Debug)]
pub struct Assignment {
    pub lhs: u64,
    pub rhs_var: u64,
    pub rhs_const: i8,
}
