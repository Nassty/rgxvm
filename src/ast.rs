#[derive(Debug, Clone)]
pub enum Ast {
    Start(Vec<Ast>),
    End(Vec<Ast>),
    Set(Vec<Ast>),
    Exclude(Vec<Ast>),
    Group(Vec<Ast>),
    Exact(String),
    NExact(String),
    Star(Box<Ast>),
    Plus(Box<Ast>),
    Question(Box<Ast>),
    Dot,
    Pipe(Box<Ast>, Box<Ast>),
}
