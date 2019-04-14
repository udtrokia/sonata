/// ## Parser
///
/// ```
/// ;Expression
/// (1 2) => Punct Literal Literal Punct
/// (a b c) => Punct Literal Literal Literal Punct
///
/// ;Operator
/// (+ 1 2) => Punct Punct Literal Literal Punct
/// (+ a (+ 1 2)) => Punct Punct Punct Punct Literal Literal Punct Punct
///
/// ;Lambda
/// ((arg) (+ arg 1))
/// => Punct Punct Literal Punct Punct Punct Literal Literal Punct Punct
///
/// ;Condition
/// ((= 1 1) (+ 1 1)
///  (= 1 1) (- 1 1))
/// =>
/// Punct Punct Punct Literal Literal Punct Punct Punct Literal Literal Punct
/// Punct Punct Literal Literal Punct Punct Punct Literal Literal Punct Punct
/// ```
