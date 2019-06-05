type Atom = &'static [u8];

mod stream {
    use super::Atom;
    /// # Stream
    /// + at   => point the end of predication
    /// + cons => convert stream to cons
    pub(super) trait Stream<T> {
        fn at<F: Fn(u8) -> bool>(self, begin: usize, predicate: F)  -> usize;
        fn cons(self) -> (&'static [T], &'static [T]);
    }

    impl Stream<u8> for &'static [u8] {
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

                    let mut _end = end;
                    _end = self.at(end, |x| !x.is_ascii_whitespace());
                    
                    (&self[begin..(end - 1)], &self[_end..])
                    // self[begin..(end - 1)].cons()
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
                x if x.is_ascii_whitespace() => {
                    let end = self.at(pos, |x| !x.is_ascii_whitespace());
                    self[end..].cons()
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
}

use stream::Stream;
/// # Cons - ((A)ddress . (D)ecrement)
/// __(1 2 3)__ means -> (cons 1 (cons 2 (cons 3 nil)))
/// 
/// # Example
/// ```rust
/// use sonata::Cons;
/// 
/// fn test_cons() {
///     let stream = b"(: > !)";
///     assert_eq!(b": > !", stream.car());          // [0]
///     assert_eq!(b":", stream.car().car());        // [0, 0]
///     assert_eq!(b"> !", stream.car().cdr());      // [0, 1]
///     assert_eq!(b">", stream.car().cdr().car());  // [0, 1, 0]
///     assert_eq!(b"!", stream.car().cdr().cdr());  // [0, 1, 1]
///     // ...
/// }
/// ```
pub trait Cons<A, D> {
    fn car(self) -> A;
    fn cdr(self) -> D;
}

impl Cons<Atom,  Atom> for &'static [u8] {
    fn car(self) -> Atom { self.cons().0 }
    fn cdr(self) -> Atom { self.cons().1 }
}

impl<A, D> Cons<A, D> for (A, D) {
    fn car(self) -> A { self.0 }
    fn cdr(self) -> D { self.1 }
}
