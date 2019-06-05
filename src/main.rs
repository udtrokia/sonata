use sonata::Cons;

fn main() {
    let stream = b"hello, world!";
    println!("ret: {:?}", stream.car());
}
