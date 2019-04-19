# sonata

Abstract unconstrained sonata in s-expr.

## Atom

```rust
/// ;expression
/// b'(symbol expr*)        ::= Eval("s-expr"),
/// b'((symbol) expr*)'     ::= Lambda("s-expr"),
/// b'((symbol-true) expr*) ::= Condition("s-expr")
///
/// ;number
/// b'd+_i8' => Number("i8"),       //::= Expr(i8)     ~> itlr | item | number
/// b'd+_u8' => Number("u8"),       //::= Expr(u8)     ~> itlr | item | number
/// b'd+_i32' => Number("i32"),     //::= Expr(i32)    ~> itlr | item | number
/// b'd+_u32' => Number("u32"),     //::= Expr(u32)    ~> itlr | item | number
/// b'd+_f32' => Number("f32"),     //::= Expr(f32)    ~> itlr | item | number
/// b'd+_i64' => Number("i64"),     //::= Expr(i64)    ~> itlr | item | number
/// b'd+_u64' => Number("u64"),     //::= Expr(u64)    ~> itlr | item | number
/// b'd+_i128' => Number("i128"),   //::= Expr(i128)   ~> itlr | item | number
/// b'd+_u128' => Number("u128"),   //::= Expr(u128)   ~> itlr | item | number
///
/// ;ident
/// b';' => Ident(";"),        //::= Expr(commnet)   ~> ident
/// b'#' => Ident("#"),        //::= Expr(super)     ~> itlr | ident
/// br#"'"# => Ident("'"),     //::= Expr(itrl)      ~> itlr | ident
///
/// ;punct
/// b'+' => Operator("+"),     //::= Expr(add)       ~> itlr | item | number
/// b'-' => Operator("-"),     //::= Expr(mins)      ~> itlr | item | number
/// b'*' => Operator("*"),     //::= Expr(multipl)   ~> itlr | item | number
/// b'/' => Operator("/"),     //::= Expr(except)    ~> itlr | item | number
/// b'>' => Operator(">"),     //::= Expr(gt)        ~> itlr | item | bool
/// b'=' => Operator("="),     //::= Expr(eq)        ~> itlr | item | bool
/// b'<' => Operator("-"),     //::= Expr(lt)        ~> itlr | item | bool
/// b'>=' => Operator(">="),   //::= Expr(ge)        ~> itlr | item | bool
/// b'<=' => Operator("<="),   //::= Expr(le)        ~> itlr | item | bool
/// b'!=' => Operator("!="),   //::= Expr(ne)        ~> itlr | item | bool
/// b'||' => Operator("||"),   //::= Expr(or)        ~> itlr | item | bool
/// b'&&' => Operator("&&"),   //::= Expr(and)       ~> itlr | item | bool
///
/// ;literal
/// b'z'  => Operator("z"),    //::= Expr(zero)      ~> itlr | item | bool
/// b'n'  => Operator("n"),    //::= Expr(non-zero)  ~> itlr | item | bool
/// b't'  => Operator("t"),    //::= Expr(true)      ~> itlr | item | bool
/// b'f'  => Operator("f"),    //::= Expr(false)     ~> itlr | item | bool
///
/// ;variable
/// b'(ltrl se-symbol)' => Variable("let"), //::= Expr(let)   ~> itlr | item | *
/// b'(#ltrl se-symbol)' => Const("const"), //::= Expr(const) ~> itlr | item | */// ;expression
```

## AST

```rust
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
```
