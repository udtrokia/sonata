mod stream;
use stream::Stream;

/// # Cons
/// (A)ddress && (D)ecrement
/// (list 1 2 3)
/// (cons 1 (cons 2 (cons 3 nil)))
pub trait Cons<A, D> {
    fn car(self) -> A;
    fn cdr(self) -> D;
}

impl Cons<&'static [u8], &'static [u8]> for &'static [u8] {
    fn car(self) -> &'static [u8] {
        self.cons().0
    }

    fn cdr(self) -> &'static [u8] {
        &self[self.cons().1..]
    }
}

#[allow(dead_code)]
mod ast;
