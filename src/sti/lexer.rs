/// # S-expr
/// expression ::= atom | list
/// atom       ::= number | symbol
/// number     ::= [+-]?['0'-'9']+
/// symbol     ::= ['A'-'Z''a'-'z'].*
/// list       ::= '(' expression* ')'
///
/// # TokenTree
/// TokenTree enumerates punct and literal
///
/// @inputs:  token stream.
/// @outputs: primitive tokentree.
macro_rules! debug {
    (
        $($tt: ident,)+
    ) => {
        $(pub struct $tt(pub &'static [u8]);)+   
        $(impl std::fmt::Debug for $tt {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", String::from_utf8(self.0.to_vec()).unwrap())
            }
        })+
    }
}
debug!{Ident, Group, Punct, Literal, }

#[derive(Debug)]
pub enum TokenTree {
    Ident(Ident),
    Group(Group),
    Punct(Punct),
    Literal(Literal)
}

/// # TokenStream
/// TokenStream handles token stream
///
/// @inputs:  stream.
/// @outputs: tokentree.
#[derive(Debug)]
pub struct TokenStream(pub &'static [u8]);

mod token_stream {
    use super::{TokenTree, Ident, Group, Punct, Literal};
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

    impl Iterator for LexerIter {
        type Item = TokenTree;
        fn next(&mut self) -> Option<TokenTree> {
            if self.0 == self.1.len() { return None; }

            let ch = self.1[self.0];
            self.0 += 1;
            
            let ret = match ch {
                b'(' | b'[' | b'{' => {
                    let begin = self.0;
                    let end = self.tok_end(|x| {
                        x == b')'|| x == b']' || x == b'}'
                    }) + 1;
                    self.0 = end;
                    self.0 = self.tok_end(|x| !x.is_ascii_whitespace());

                    Some(TokenTree::Group(Group(&self.1[(begin -1)..end])))
                },
                b'"' => {
                    let begin = self.0;
                    let end = self.tok_end(|x| x == b'"') + 1;
                    self.0 = end;
                    self.0 = self.tok_end(|x| !x.is_ascii_whitespace());

                    Some(TokenTree::Literal(Literal(&self.1[(begin -1)..end])))
                },
                b';' => {
                    let begin = self.0;
                    let end = self.tok_end(|x| x == b'\n');
                    self.0 = end;
                    self.0 = self.tok_end(|x| !x.is_ascii_whitespace());

                    Some(TokenTree::Ident(Ident(&self.1[(begin -1)..end])))
                },
                x if x.is_ascii_alphanumeric() => {
                    let begin = self.0;
                    let end = self.tok_end(|x| x.is_ascii_whitespace());
                    self.0 = end;
                    self.0 = self.tok_end(|x| !x.is_ascii_whitespace());
                    
                    Some(TokenTree::Literal(Literal(&self.1[(begin-1)..end])))
                },
                x if x.is_ascii_punctuation() => {
                    let begin = self.0;
                    let end = self.tok_end(|x| x.is_ascii_whitespace());
                    self.0 = end;
                    self.0 = self.tok_end(|x| !x.is_ascii_whitespace());
                    
                    Some(TokenTree::Punct(Punct(&self.1[(begin-1)..end])))
                },
                _ => {
                    let begin = self.0;
                    let end = self.tok_end(|x| x.is_ascii_whitespace());
                    self.0 = end;
                    self.0 = self.tok_end(|x| !x.is_ascii_whitespace());
                    
                    Some(TokenTree::Literal(Literal(&self.1[(begin-1)..end])))
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

impl TokenStream {
    pub fn spwan<F: Fn(TokenTree)>(self, cb: F) {
        for tt in self.into_iter() { cb(tt) }
    }
}
