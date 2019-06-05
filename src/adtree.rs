use cons::Cons;

/// # Address-Decrement Tree
/// kind of binary tree, implementend without `struct`.
/// 
/// # Examples
/// ```
/// let cons_set = [
///   (00, 01),
///   (02, 03), (04, 05),
///   (06, 07), (08, 09), (10, 11), (12, 13),
/// ];
/// 
/// let adtree = [
///   (0, vec![0]), (1, vec![1]),
///   (2, vec![0, 0]), (3, vec![0, 1]), (4, vec![1, 0]), (5, vec![1, 1]),
///   (6, vec![0, 0, 0]), (7, vec![0, 0, 1]), (8, vec![0, 1, 0]), (9, vec![0, 1, 1]), 
///   (10, vec![1, 0, 0]), (11, vec![1, 0, 1]), (12, vec![1, 1, 0]), (13, vec![1, 1, 1])
/// ];
/// 
/// let mut sonata = "(: hello)";
/// // ": hello" -> [0]
/// // ":" -> [0, 0]
/// // "hello" -> [0, 1]
///
/// sonata = "(: hello world)";
/// // ": hello world" -> [0]
/// // ":" -> [0, 0]
/// // "hello" -> [0, 1, 0]
/// // "world" -> [0, 1, 1]
/// ```
pub(crate) trait ADTree {
    fn height() -> usize;
}
