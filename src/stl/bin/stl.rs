extern crate stl;
use stl::Cons;
fn main() {
    let stream = b": hello";
    println!("{:?}", stream.cdr());
}
