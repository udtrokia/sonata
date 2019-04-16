extern crate sti;
extern crate dirs;

use std::io::prelude::*;
use std::fs::File;
use sti::TokenStream;

fn main() {
    let mut st = dirs::home_dir().unwrap();
    st.push("tmp/test.st");

    let mut stream = File::open(st).unwrap();
    let mut content = vec![];
    stream.read_to_end(&mut content).unwrap();

    let ts = TokenStream(
        Box::leak(
            content.to_owned().into_boxed_slice()
        )
    );

    ts.spwan(|x| println!("{:?}", x));
}
