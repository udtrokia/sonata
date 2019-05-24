// for Stream
pub(crate) type Atom = &'static [u8];

// for AST
pub(crate) type Num = (&'static [u8], &'static [u8]);

// for ADTree
pub(crate) type AD = (Address, Decrement);
pub(crate) type Address = (&'static [u8], &'static [u8]);
pub(crate) type Decrement = (&'static [u8], &'static [u8]);
