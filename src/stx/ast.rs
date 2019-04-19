/// # AST
/// (1 1)       ::= List(Atom Atom)
/// (1 (1 1))   ::= List(Atom List(Atom Atom))
/// (1 (1 1) 1) ::= List(Atom List(Atom Atom) Atom)
use std::collections::BTreeSet;
use crate::lexer::TokenStream;
pub enum AST {
    Define(&'static [u8], &'static [u8]),
    Lambda(&'static [u8], &'static [u8]),
    Condition(BTreeSet<&'static [u8]>),
}

pub struct Expr(TokenStream);
