// lexer.rs

/// # Token Stream
pub mod token_stream {
    #[derive(Clone)]
    pub struct TokenStream(&'static [u8]);

    impl TokenStream {
        pub fn new(raw: &'static [u8]) -> TokenStream {
            TokenStream(raw)
        }

        pub fn is_empty(self) -> bool {
            match self.0.len() {
                0 => true,
                _ => false
            }
        }
    }

    impl ToString for TokenStream {
        fn to_string(&self) -> String {
            std::str::from_utf8(self.0).unwrap().to_string()
        }
    }
}

/// # TokenTree
pub mod token_tree {
    use crate::lexer::token_stream::TokenStream;

    pub enum TokenTree {
        List(List),
        Atom(Atom)
    }

    /// ## List
    pub struct List(TokenStream);

    /// ## Atom
    pub struct Ident;

    enum Spacing {
        Alone,
        Joint
    }

    pub struct Punct;
    pub struct Literal;

    pub enum Atom {
        Ident(Ident),
        Punct(Punct),
        Literal(Literal)
    }
}
