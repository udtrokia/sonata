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

impl<A, D> ConsTrait<A, D> for (A, D) {
    fn car(self) -> A {
        self.0
    }

    fn cdr(self) -> D {
        self.1
    }
}
