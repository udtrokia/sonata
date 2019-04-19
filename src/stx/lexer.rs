/// # TokenTree
/// TokenTree enumerates punct and literal
/// 
/// @inputs:  token stream.
/// @outputs: primitive tokentree.
#[derive(Debug)]
pub enum TokenTree {
    Atom(&'static [u8]),
    List(TokenStream),
}

impl std::fmt::Display for TokenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (x, n) = match &self {
            TokenTree::Atom(v) => (v, "Atom"),
            TokenTree::List(TokenStream(v)) => (v, "List")
        };
        write!(f, "{}({})", n, String::from_utf8(x.to_vec()).unwrap())
    }
}

/// # TokenStream
/// TokenStream handles token stream
///
/// @inputs:  stream.
/// @outputs: tokentree.
#[derive(Debug)]
pub struct TokenStream(pub &'static [u8]);

mod token_stream {
    pub struct LexerIter( pub usize, pub &'static [u8]);

    impl LexerIter {
        fn tok_end<'a, F: Fn(u8) -> bool>(&self, predicate: F) -> usize {
            let raw = self.1;
            let begin = self.0;
            raw[begin..].iter().enumerate()
                .find(|(_, &x)| predicate(x))
                .map(|(i, _)| begin + i)
                .unwrap_or(raw.len())
        }
    }

    use super::{TokenTree, TokenStream};
    impl Iterator for LexerIter {
        type Item = TokenTree;
        // TODO: Error handlers
        fn next(&mut self) -> Option<TokenTree> {
            if self.0 == self.1.len() { return None; }

            let ch = self.1[self.0];
            self.0 += 1;

            let ret = match ch {
                b'(' => {
                    let begin = self.0;
                    let mut next = self.tok_end(|x| x == b'(') + 1;
                    let mut end = self.tok_end(|x| x == b')') + 1;
                    while end > next {
                        self.0 = end;
                        next = self.tok_end(|x| x == b'(') + 1;
                        end = self.tok_end(|x| x == b')') + 1;
                    }
                    self.0 = end;
                    self.0 = self.tok_end(|x| !x.is_ascii_whitespace());
                    
                    Some(TokenTree::List(TokenStream(&self.1[(begin -1)..end])))
                },
                _ => {
                    let begin = self.0;
                    let end = self.tok_end(|x| x.is_ascii_whitespace());
                    self.0 = end;
                    self.0 = self.tok_end(|x| !x.is_ascii_whitespace());

                    Some(TokenTree::Atom(&self.1[(begin-1)..end]))
                },
            };
            ret
        }
    }
}

use token_stream::LexerIter;
impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = LexerIter;

    fn into_iter(self) -> Self::IntoIter {
        LexerIter(0, self.0).into_iter()
    }
}

impl ToString for TokenStream {
    fn to_string(&self) -> String {
        String::from_utf8(self.0.to_vec()).unwrap()
    }
}

impl From<TokenTree> for TokenStream {
    fn from(list: TokenTree) -> Self {
        match list {
            TokenTree::Atom(ts) => TokenStream(ts),
            TokenTree::List(ts) => ts
        }
    }
}

impl TokenStream {
    pub fn spwan(self) -> Vec<TokenTree> {
        self.into_iter().collect()
    }
}
