#[derive(Debug)]
pub enum Lexeme {
    Var,
    Print,
    If,
    Else,
    Add,
    Sub,
    Mul,
    Div,
    End,
    Loop,
    Array,
    String,
    Endstring,
    Sqrt,
    Float,
    Function,
    Call,
    Struct,
    Endstruct,
    AddF,
    SubF,
    MulF,
    DivF,
    ABS,
    POW,
}

impl Lexeme {
    pub fn from_str(s: &str) -> Self {
        match s {
            "var" => Lexeme::Var,
            "print" => Lexeme::Print,
            "if" => Lexeme::If,
            "else" => Lexeme::Else,
            "add" => Lexeme::Add,
            "sub" => Lexeme::Sub,
            "mul" => Lexeme::Mul,
            "div" => Lexeme::Div,
            "end" => Lexeme::End,
            "loop" => Lexeme::Loop,
            "array" => Lexeme::Array,
            "string" => Lexeme::String,
            "endstring" => Lexeme::Endstring,
            "sqrt" => Lexeme::Sqrt,
            "float" => Lexeme::Float,
            "function" => Lexeme::Function,
            "call" => Lexeme::Call,
            "struct" => Lexeme::Struct,
            "endstruct" => Lexeme::Endstruct,
            "add_f" => Lexeme::AddF,
            "sub_f" => Lexeme::SubF,
            "mul_f" => Lexeme::MulF,
            "div_f" => Lexeme::DivF,
            "abs" => Lexeme::ABS,
            "pow" => Lexeme::POW,
            _ => panic!("Invalid lexeme: {}", s),
        }
    }
}

pub enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl Comparison {
    pub fn from_str(s: &str) -> Self {
        match s {
            "==" => Comparison::Equal,
            "!=" => Comparison::NotEqual,
            "<" => Comparison::LessThan,
            "<=" => Comparison::LessThanOrEqual,
            ">" => Comparison::GreaterThan,
            ">=" => Comparison::GreaterThanOrEqual,
            _ => panic!("Invalid comparison operator: {}", s),
        }
    }
}
