#![allow(dead_code)]
mod error;
mod lexer;
use std::io::Read;
use std::fs::File;
pub struct Stx(pub std::path::PathBuf);
impl Stx {
    pub fn source(self) -> lexer::TokenStream  {
        let mut stream = File::open(self.0).unwrap();
        let mut content = vec![];
        stream.read_to_end(&mut content).unwrap();

        lexer::TokenStream(
            Box::leak(content.to_owned().into_boxed_slice())
        )
    }

    // batch
    // + list
    // + llst
    //  + symbol
    //  + list
    // pub fn batch(&self) -> Vec<lexer::List> {
    //     let (tx, rx) = std::sync::mpsc::channel();
    // 
    //     &self.source().spwan(move |x| {
    //         match x {
    //             lexer::TokenTree::List(lexer::List(l)) => tx.send(l).unwrap(),
    //             lexer::TokenTree::Number(lexer::Number(l)) => tx.send(l).unwrap(),
    //             lexer::TokenTree::Symbol(lexer::Symbol(l)) => tx.send(l).unwrap()
    //         }
    //     });
    // 
    //     let v: Vec<_> = rx.iter().collect();
    //     
    // }
}

mod atom;
mod ast;
