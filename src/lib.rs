/*!
Mint - Math interoperability standard types.

Defines basic math types useful for computer graphics.
Designed to serve as an interoperability standard between libraries.
*/
#![no_std]
#![deny(
    missing_docs,
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all
)]

mod into_mint;
mod matrix;
mod rotation;
mod vector;

pub use into_mint::*;
pub use matrix::*;
pub use rotation::*;
pub use vector::*;
