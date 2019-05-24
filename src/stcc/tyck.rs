// for Stream
pub(super) type Atom = &'static [u8];

// for AST
pub(super) type Num = (&'static [u8], &'static [u8]);

// for ADTree
pub(super) type AD = (Address, Decrement);
pub(super) type Address = (&'static [u8], &'static [u8]);
pub(super) type Decrement = (&'static [u8], &'static [u8]);
