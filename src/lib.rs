/*!
Mint - Math interoperability standard types.

Defines basic math types useful for computer graphics.
Designed to serve as an interoperability standard between libraries.
*/
#![no_std]
#![deny(missing_docs)]

mod matrix;
mod rotation;
mod vector;

pub use matrix::*;
pub use rotation::*;
pub use vector::*;
