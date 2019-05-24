use crate::tyck::Atom;
/// # Stream
/// + at   => point the end of predication
/// + cons => convert stream to cons
trait StreamTrait<T> {
    fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize;
    fn cons(self) -> (&'static [T], &'static [T]);
}

impl StreamTrait<u8> for &'static [u8] {
    fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize {
        self[begin..].iter().enumerate()
            .find(|(_, &x)| predicate(x))
            .map(|(i, _)| begin + i)
            .unwrap_or(self.len())
    }
    
    fn cons(self) -> (Atom, Atom) {
        if self.len() == 0 {
            return (b"", b"");
        }
        
        let mut pos = 1;
        match self[0] {
            b'(' => {
                let begin = pos;
                let mut next = self.at(pos, |x| x == b'(') + 1;
                let mut end = self.at(pos, |x| x == b')') + 1;
                while end > next {
                    pos = end;
                    next = self.at(pos, |x| x == b'(') + 1;
                    end = self.at(pos, |x| x == b')') + 1;
                }

                self[begin..(end - 1)].cons()
            },
            b'"' => {
                let begin = pos;
                let end = self.at(pos, |x| x == b'"');
                let _end = self.at(end, |x| !x.is_ascii_whitespace());

                (&self[(begin - 1)..end], &self[_end..])
            },
            b';' => {
                let end = self.at(pos, |x| x == b'\n');
                (&self[(pos - 1)..end], &self[end..])
            },
            _ => {
                let begin = pos;
                let end = self.at(pos, |x| x.is_ascii_whitespace());
                let mut _end = self.at(end, |x| !x.is_ascii_whitespace());

                (&self[(begin - 1)..end], &self[_end..])
            }
        }
    }
}

/// High level Stream.
/// # Cons - (A)ddress && (D)ecrement
/// __(1 2 3)__ means: (cons 1 (cons 2 (cons 3 nil)))
pub(crate) mod cons {
    use crate::tyck::Atom;
    use crate::stream::StreamTrait;

    /// # Example
    /// ```rust
    /// use stcc::Stcc;
    /// 
    /// #[test]
    /// fn test_cons() {
    ///     let stream = b"(: hello)";    
    ///     assert_eq!(b":", stream.car());
    ///     assert_eq!(b"hello", stream.cdr());
    ///     // ...
    /// }
    /// ```
    pub trait ConsTrait<A, D> {
        fn car(self) -> A;
        fn cdr(self) -> D;
    }

    impl ConsTrait<Atom,  Atom> for &'static [u8] {
        fn car(self) -> Atom {
            self.cons().0
        }

        fn cdr(self) -> Atom {
            self.cons().1
        }
    }
}
pub use cons::ConsTrait;
