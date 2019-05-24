use super::tyck::Atom;
use super::stream::StreamTrait;

/// # Cons - ((A)ddress . (D)ecrement)
/// __(1 2 3)__ means -> (cons 1 (cons 2 (cons 3 nil)))
/// 
/// # Example
/// ```rust
/// use sonata::stcc::Cons;
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

impl<A, D> ConsTrait<A, D> for (A, D) {
    fn car(self) -> A {
        self.0
    }

    fn cdr(self) -> D {
        self.1
    }
}
