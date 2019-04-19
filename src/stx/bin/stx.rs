extern crate sti;
extern crate dirs;

use sti::Stx;
fn main() {
    let mut st = dirs::home_dir().unwrap();
    st.push("tmp/test.st");
    let stream = Stx(st).source();
    stream.spwan().iter().for_each(|x| println!("{}", x));
}
