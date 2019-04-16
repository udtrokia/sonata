/// ;expression
/// b'(symbol expr*)        ::= Eval("s-expr"),
/// b'((symbol) expr*)'     ::= Lambda("s-expr"),
/// b'((symbol-true) expr*) ::= Condition("s-expr")
enum Expr {
    Eval,
    Lambda,
    Condition
}


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
enum Number {
    I8,
    U8,
    F32,
    I32,
    U32,
    F64,
    I64,
    U64,
    I128,
    U128,
    I256,
    U256,
}

/// ;ident
/// b';' => Ident(";"),        //::= Expr(commnet)   ~> ident
/// b'#' => Ident("#"),        //::= Expr(super)     ~> itlr | ident
/// br#"'"# => Ident("'"),     //::= Expr(itrl)      ~> itlr | ident
enum Ident {
    Super,
    Comment,
    Iterial,
}

/// ;punct
/// b'+' => Operator("+"),     //::= Expr(add)       ~> itlr | item | number
/// b'-' => Operator("-"),     //::= Expr(minus)     ~> itlr | item | number
/// b'*' => Operator("*"),     //::= Expr(multiply)  ~> itlr | item | number
/// b'/' => Operator("/"),     //::= Expr(except)    ~> itlr | item | number
/// b'>' => Operator(">"),     //::= Expr(gt)        ~> itlr | item | bool
/// b'=' => Operator("="),     //::= Expr(eq)        ~> itlr | item | bool
/// b'<' => Operator("-"),     //::= Expr(lt)        ~> itlr | item | bool
/// b'>=' => Operator(">="),   //::= Expr(ge)        ~> itlr | item | bool
/// b'<=' => Operator("<="),   //::= Expr(le)        ~> itlr | item | bool
/// b'!=' => Operator("!="),   //::= Expr(ne)        ~> itlr | item | bool
/// b'||' => Operator("||"),   //::= Expr(or)        ~> itlr | item | bool
/// b'&&' => Operator("&&"),   //::= Expr(and)       ~> itlr | item | bool
enum Punct {
    Add,
    Minus,
    Multiply,
    Except,
    Gt,
    Eq,
    Lt,
    Ge,
    Le,
    Ne,
    Or,
    And,
}

/// ;literal
/// b'z'  => Operator("z"),    //::= Expr(zero)      ~> itlr | item | bool
/// b'n'  => Operator("n"),    //::= Expr(non-zero)  ~> itlr | item | bool
/// b't'  => Operator("t"),    //::= Expr(true)      ~> itlr | item | bool
/// b'f'  => Operator("f"),    //::= Expr(false)     ~> itlr | item | bool
enum Literal {
    Zero,
    Nil,
    True,
    False
}

/// ;variable
/// b'(ltrl se-symbol)' => Variable("let"), //::= Expr(let)   ~> itlr | item | *
/// b'(#ltrl se-symbol)' => Const("const"), //::= Expr(const) ~> itlr | item | *
enum Variable {
    Let,
    Const
}
