#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
mod tyck;
mod stream;

/// High level Stream.
/// # Cons - (A)ddress && (D)ecrement
/// __(1 2 3)__ means: (cons 1 (cons 2 (cons 3 nil)))
mod cons;
pub use cons::ConsTrait as Stcc;

mod adtree;
mod ast;
