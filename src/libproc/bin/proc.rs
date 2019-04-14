extern crate libproc;
extern crate dirs;

use std::io::prelude::*;
use std::fs::File;
use libproc::TokenStream;

fn main() {
    let mut st = dirs::home_dir().unwrap();
    st.push("tmp/test.st");

    let mut stream = File::open(st).unwrap();
    let mut content = vec![];
    stream.read_to_end(&mut content).unwrap();
    
    let ts = TokenStream::new(&content);

    println!("{:?}", String::from_utf8(content.to_owned()).unwrap());
    
    for tt in ts.into_iter() {
        match tt {
            _ => println!("{:?}", tt)
        }
    }
}
