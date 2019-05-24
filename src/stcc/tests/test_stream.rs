use stcc::Stcc;

#[test]
fn test_cons() {
    let stream = b"(: hello)";    
    assert_eq!(b":", stream.car());
    assert_eq!(b"hello", stream.cdr());
    
    assert_eq!(b":", stream.car().car());
    assert_eq!(b"", stream.car().cdr());
    assert_eq!(b"hello", stream.cdr().car());
    assert_eq!(b"", stream.cdr().cdr());
}
