use stcc::Stcc;

fn main() {
    let stream = b"(: (: hello))";

    println!("car: {:?}", String::from_utf8(stream.car().to_vec()).unwrap());
    println!("cdr: {:?}", String::from_utf8(stream.cdr().to_vec()).unwrap());
    println!();
    println!("caar: {:?}", String::from_utf8(stream.car().car().to_vec()).unwrap());
    println!("cadr: {:?}", String::from_utf8(stream.car().cdr().to_vec()).unwrap());
    println!();
    println!("cdar: {:?}", String::from_utf8(stream.cdr().car().to_vec()).unwrap());
    println!("cddr: {:?}", String::from_utf8(stream.cdr().cdr().to_vec()).unwrap());
}
