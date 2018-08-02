/*!
Mint - Math interoperability standard types.

Defines basic math types useful for computer graphics.
Designed to serve as an interoperability standard between libraries.
*/
#![deny(missing_docs)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod matrix;
mod rotation;
mod vector;
mod tests;

pub use matrix::*;
pub use rotation::*;
pub use vector::*;
