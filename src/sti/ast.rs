/// expression ::= atom | list
/// atom       ::= number | symbol
/// number     ::= [+-]?['0'-'9']+
/// symbol     ::= ['A'-'Z''a'-'z'].*
/// list       ::= '(' expression* ')'
///
/// ;expression
/// (1 2)                ::= cons | number
/// (a 1)                ::= expr | define
/// (+ 1 2)              ::= expr | operate
/// ((arg) (+ arg 1))    ::= expr | lambda
/// ((= 1 1) (+ 1 1)))   ::= expr | condition
///
/// ;number
/// (x 1)                ::= expr | define | i32
/// (x 1_u32)            ::= expr | define | u32
/// (x 1_f64)            ::= expr | define | f64
/// 
/// ;symbol
/// (x @_@)              ::= cons | symbol
/// (x 1)                ::= expr | defvar
/// (x (x) (+ x 1))      ::= expr | defunc
///
/// ;ident
/// symbol(;)            ::= ident | comment
/// symbol(')            ::= ident | literal
/// symbol(")            ::= ident | string
/// symbol(#)            ::= ident | super
enum Atom {
    Number,
    Symbol,
}

struct Cons(Atom, Atom);
struct Expr(Atom, Vec<Cons>);
