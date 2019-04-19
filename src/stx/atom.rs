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
#[derive(Debug, PartialEq)]
pub struct Num(pub &'static str, pub &'static str);

impl From<&'static str> for Num {
    fn from(atom: &'static str) -> Self {
        let o = atom.find('_');
        match o.is_none() {
            true => Num(atom, "i32"),
            false => Num(&atom[..o.unwrap()], &atom[(o.unwrap() + 1)..])
        }
        
    }
}

#[test]
fn num_from() {
    assert_ne!(Num::from("0_i32"), Num("0", "u32"));
    assert_ne!(Num::from("1_i32"), Num("0", "i32"));
    assert_eq!(Num::from("0_i32"), Num("0", "i32"));
    assert_eq!(Num::from("0_u32"), Num("0", "u32"));
    assert_eq!(Num::from("0_f32"), Num("0", "f32"));
    assert_eq!(Num::from("0_i64"), Num("0", "i64"));
    assert_eq!(Num::from("0_u64"), Num("0", "u64"));
    assert_eq!(Num::from("0_f64"), Num("0", "f64"));
    assert_eq!(Num::from("0_i128"), Num("0", "i128"));
    assert_eq!(Num::from("0_u128"), Num("0", "u128"));
    assert_eq!(Num::from("0_i256"), Num("0", "i256"));
    assert_eq!(Num::from("0_u256"), Num("0", "u256"));
}
