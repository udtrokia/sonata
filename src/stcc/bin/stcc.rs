use stcc::Cons;

fn main() {
    let stream = b"(: hello)";

    println!("{:?}", stream.car());
}
