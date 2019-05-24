/// # Stream
/// + at   => point the end of predication
/// + cons => convert stream to cons
trait StreamTrait<T> {
    fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize;
    fn cons(self) -> (&'static [T], usize);
}

impl StreamTrait<u8> for &'static [u8] {
    fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize {
        self[begin..].iter().enumerate()
            .find(|(_, &x)| predicate(x))
            .map(|(i, _)| begin + i)
            .unwrap_or(self.len())
    }
    
    fn cons(self) -> (&'static [u8], usize) {
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

                (&self[(begin - 1)..end], end)
            },
            b'"' => {
                let begin = pos;
                let end = self.at(pos, |x| x == b'"');
                let _end = self.at(end, |x| !x.is_ascii_whitespace());

                (&self[(begin - 1)..end], _end)
            },
            b';' => {
                let end = self.at(pos, |x| x == b'\n');
                (&self[(pos - 1)..end], end)
            },
            _ => {
                let begin = pos;
                let end = self.at(pos, |x| x.is_ascii_whitespace());
                let _end = self.at(end, |x| !x.is_ascii_whitespace());

                (&self[(begin - 1)..end], _end)
            }
        }
    }
}

/// High level Stream.
pub(crate) mod cons {
    use crate::stream::StreamTrait;
    /// # Cons - (A)ddress && (D)ecrement
    /// (list 1 2 3)  
    /// (cons 1 (cons 2 (cons 3 nil)))
    pub trait ConsTrait<A, D> {
        fn car(self) -> A;
        fn cdr(self) -> D;
    }

    impl ConsTrait<&'static [u8], &'static [u8]> for &'static [u8] {
        fn car(self) -> &'static [u8] {
            self.cons().0
        }

        fn cdr(self) -> &'static [u8] {
            &self[self.cons().1..]
        }
    }
}
pub use cons::ConsTrait;
