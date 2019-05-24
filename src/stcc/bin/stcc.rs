use stcc::Stcc;

fn main() {
    let stream = b"(: > foo)";

    println!("car: {:?}", String::from_utf8(stream.car().to_vec()).unwrap());
    println!("cdr: {:?}", String::from_utf8(stream.cdr().to_vec()).unwrap());
    println!();
    println!("caar: {:?}", String::from_utf8(stream.car().car().to_vec()).unwrap());
    println!("cadr: {:?}", String::from_utf8(stream.car().cdr().to_vec()).unwrap());
    println!();
    println!("caaar: {:?}", String::from_utf8(stream.car().car().car().to_vec()).unwrap());
    println!("cadar: {:?}", String::from_utf8(stream.car().cdr().car().to_vec()).unwrap());
    println!("caddr: {:?}", String::from_utf8(stream.car().cdr().cdr().to_vec()).unwrap());
    println!();
    println!("cadaar: {:?}", String::from_utf8(stream.car().cdr().car().car().to_vec()).unwrap());
    println!("caddar: {:?}", String::from_utf8(stream.car().cdr().cdr().car().to_vec()).unwrap());
}
