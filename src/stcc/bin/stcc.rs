use stcc::Stream;

fn main() {
    let stream = b"(: hello)";

    println!("{:?}", stream.car());
}
