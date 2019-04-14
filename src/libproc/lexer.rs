/// expression ::= atom | list
/// atom       ::= number | symbol
/// number     ::= [+-]?['0'-'9']+
/// symbol     ::= ['A'-'Z''a'-'z'].*
/// list       ::= '(' expression* ')'

pub mod token_tree {
    use crate::TokenStream;

    #[derive(Debug)]
    pub enum TokenTree {
        Group,
        Ident,
        Punct,
        Literal,
    }

    pub enum Spacing {
        Alone,
        Joint,
    }
}
pub use token_tree::*;

mod token_stream {
    use crate::TokenTree;
    use crate::token_stream_iter::TokenStreamIter;

    #[derive(Debug)]
    pub struct TokenStream<'ts>(pub &'ts [u8]);
    
    impl<'ts> TokenStream<'ts> {
        pub fn new(raw: &'ts [u8]) -> TokenStream<'ts> {
            TokenStream(raw)
        }

        pub fn is_empty(self) -> bool {
            match self.0.len() {
                0 => true,
                _ => false
            }
        }
    }
    
    impl<'ts> ToString for TokenStream<'ts> {
        fn to_string(&self) -> String {
            ::std::str::from_utf8(self.0).unwrap().to_string()
        }
    }

    impl<'ts> IntoIterator for TokenStream<'ts> {

        type Item = TokenTree;
        type IntoIter = TokenStreamIter<'ts>;

        fn into_iter(self) -> TokenStreamIter<'ts> {
            TokenStreamIter::new(self).into_iter()
        }
    }
}
pub use token_stream::TokenStream;

mod token_stream_iter {
    use crate::TokenStream;
    use crate::TokenTree;

    pub struct TokenStreamIter<'tsi> {
        pos: usize,
        raw: &'tsi [u8]
    }

    impl<'tsi> TokenStreamIter<'tsi> {
        pub fn new(stream: TokenStream<'tsi>) -> TokenStreamIter<'tsi> {
            TokenStreamIter {
                pos: 0,
                raw: stream.0
            }
        }
    }

    impl<'tsi> Iterator for TokenStreamIter<'tsi> {
        type Item = TokenTree;

        fn next(&mut self) -> Option<TokenTree> {
            if self.pos == self.raw.len() {
                return None;
            }

            let ch = self.raw[self.pos];
            self.pos += 1;

            let ret = match ch {
                x if x.is_ascii_punctuation() => {
                    Some(TokenTree::Punct)
                },
                _ => Some(TokenTree::Ident)
            };
            ret
        }
    }
}
