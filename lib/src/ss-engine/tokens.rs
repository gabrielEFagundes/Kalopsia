#[allow(non_camel_case_types)]
pub enum Tokens{
    Ident,

    R_Paren, // (
    L_Paren, // )
    R_Brace, // {
    L_Brace, // }
    R_Bracket, // [
    L_Bracket, // ]

    Quotes,
    Equals
}

impl Tokens{
    pub fn tokenize<T>(val: T){}
}