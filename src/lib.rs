/*!
Mint - Math interoperability standard types.

Defines basic math types useful for computer graphics.
Designed to serve as an interoperability standard between libraries.
*/
#![deny(missing_docs)]

mod matrix;
mod rotation;
mod vector;

pub use matrix::*;
pub use rotation::*;
pub use vector::*;

/// Conversion directly between types that implement `From` for the same `mint` type.
pub trait Convert: Sized {
    /// The corresponding `mint` type
    type Via: From<Self>;
    /// Convert into an equivalent type
    fn convert<T: From<Self::Via>>(self) -> T {
        T::from(Self::Via::from(self))
    }
}
