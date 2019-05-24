use sonata::Cons;

#[test]
fn test_cons() {
    let stream = b"(: hello)";    
    assert_eq!(b": hello", stream.car());
    assert_eq!(b"", stream.cdr());
    assert_eq!(b":", stream.car().car());
    assert_eq!(b"hello", stream.car().cdr());
    assert_eq!(b"", stream.cdr().car());
    assert_eq!(b"", stream.cdr().cdr());
}

#[test]
fn test_list() {
    let stream = b"(: > !)";
    assert_eq!(b": > !", stream.car());          // [0]
    assert_eq!(b":", stream.car().car());        // [0, 0]
    assert_eq!(b"> !", stream.car().cdr());      // [0, 1]
    assert_eq!(b">", stream.car().cdr().car());  // [0, 1, 0]
    assert_eq!(b"!", stream.car().cdr().cdr());  // [0, 1, 1]
}
