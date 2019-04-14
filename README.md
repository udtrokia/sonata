# Sonata

Abstract unconstrained sonata in s-expr.

## Bridge

```rust
/// ;Expression
/// (1 2) => Punct Literal Literal Punct
/// (a b c) => Punct Literal Literal Literal Punct
/// 
/// ;Operator
/// (+ 1 2) => Punct Punct Literal Literal Punct
/// (+ a (+ 1 2)) => Punct Punct Punct Punct Literal Literal Punct Punct
/// 
/// ;Lambda
/// ((arg) (+ arg 1))
/// => Punct Punct Literal Punct Punct Punct Literal Literal Punct Punct
/// 
/// ;Condition
/// ((= 1 1) (+ 1 1)
///  (= 1 1) (- 1 1))
/// =>
/// Punct Punct Punct Literal Literal Punct Punct Punct Literal Literal Punct
/// Punct Punct Literal Literal Punct Punct Punct Literal Literal Punct Punct

enum Punct { Atom, List }
enum Literal { Number, Symbol }
```

## Lexer

```rust
/// expression ::= atom | list
/// atom       ::= number | symbol
/// number     ::= [+-]?['0'-'9']+
/// symbol     ::= ['A'-'Z''a'-'z'].*
/// list       ::= '(' expression* ')'

match stream.spawn() {
    b'(...)' => Expr | List
    b'[0-9]+' => Number
    b'[A-Za-z].*' => Symbol
    _ => Atom
}

mod expr {
    enum Expr { Atom, List }

    impl Expr {
        fn is_atom(self) -> bool;
        fn is_list(self) -> bool;
    }
}

mod atom {
    enum Atom { Number, Symbol }

    impl Atom {
        fn is_number(self) -> bool;
        fn is_symbol(self) -> bool;
    }
}

mod list {
    struct List(Vec<Expr>);

    impl List {
        fn is_expr(self) -> bool;
        fn is_ltrl(self) -> bool;
        fn eval(self) -> Result<(), Err>;
    }
}
```

## AST

```rust
enum Expression {
    Eval,
    Lambda,
    Condition,
}

enum Number {
    Float,
    Integer,
}

enum Symbol {
    Ident,
    Literal,
    Operator,
    Variable,
}
```

## Proc

```rust
match expr {
    b'(symbol expr*) => Eval("s-expr"),
    b'((symbol) expr*)' => Lambda("s-expr"),
    b'((symbol-true) expr*) => Condition("s-expr")
}

match Number {
    b'd+_i8' => Number("i8"),       //::= Expr(i8)     ~> itlr | item | number
    b'd+_u8' => Number("u8"),       //::= Expr(u8)     ~> itlr | item | number
    b'd+_i32' => Number("i32"),     //::= Expr(i32)    ~> itlr | item | number
    b'd+_u32' => Number("u32"),     //::= Expr(u32)    ~> itlr | item | number
    b'd+_f32' => Number("f32"),     //::= Expr(f32)    ~> itlr | item | number
    b'd+_i64' => Number("i64"),     //::= Expr(i64)    ~> itlr | item | number
    b'd+_u64' => Number("u64"),     //::= Expr(u64)    ~> itlr | item | number
    b'd+_i128' => Number("i128"),   //::= Expr(i128)   ~> itlr | item | number
    b'd+_u128' => Number("u128"),   //::= Expr(u128)   ~> itlr | item | number
}

match symbol {
    // Ident
    b'#' => Ident("#"),        //::= Expr(super)     ~> itlr | ident
    b';' => Ident(";"),        //::= Expr(commnet)   ~> itlr | ident
    br#"'"# => Ident("'"),     //::= Expr(itrl)      ~> itlr | ident

    // Operator
    b'+' => Operator("+"),     //::= Expr(add)       ~> itlr | item | number
    b'-' => Operator("-"),     //::= Expr(minus)     ~> itlr | item | number
    b'*' => Operator("*"),     //::= Expr(multiply)  ~> itlr | item | number
    b'/' => Operator("/"),     //::= Expr(except)    ~> itlr | item | number
    b'>' => Operator(">"),     //::= Expr(gt)        ~> itlr | item | bool
    b'=' => Operator("="),     //::= Expr(eq)        ~> itlr | item | bool
    b'<' => Operator("-"),     //::= Expr(lt)        ~> itlr | item | bool
    b'>=' => Operator(">="),   //::= Expr(ge)        ~> itlr | item | bool
    b'<=' => Operator("<="),   //::= Expr(le)        ~> itlr | item | bool
    b'!=' => Operator("!="),   //::= Expr(ne)        ~> itlr | item | bool
    b'||' => Operator("||"),   //::= Expr(or)        ~> itlr | item | bool
    b'&&' => Operator("&&"),   //::= Expr(and)       ~> itlr | item | bool

    // w-Operator
    b'z'  => Operator("z"),    //::= Expr(zero)      ~> itlr | item | bool
    b'n'  => Operator("n"),    //::= Expr(non-zero)  ~> itlr | item | bool
    b't'  => Operator("t"),    //::= Expr(true)      ~> itlr | item | bool
    b'f'  => Operator("f"),    //::= Expr(false)     ~> itlr | item | bool

    // Variable
    b'(ltrl se-symbol)' => Variable("let"), //::= Expr(let)   ~> itlr | item | *
    b'(#ltrl se-symbol)' => Const("const"), //::= Expr(const) ~> itlr | item | *

    // Literal
    _ => literal(*),  //::==Expr(itrl)  ~> itlr
}
```
