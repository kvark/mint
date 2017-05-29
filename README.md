# mint
[![Build Status](https://travis-ci.org/kvark/mint.svg)](https://travis-ci.org/kvark/mint)
[![Docs](https://docs.rs/mint/badge.svg)](https://docs.rs/mint)
[![Crates.io](https://img.shields.io/crates/v/mint.svg?maxAge=2592000)](https://crates.io/crates/mint)

Math Interoperability Types.

This library contains type aliases of common math types used in computer graphics, to Rust built-in types.
It's only purpose is to serve as a standard and interoperability language between various components of rust-gamedev ecosystem that happen to expose math-related types on their API.
Since there are no new types/traits here, users are not even required to depend on this library in order to take the benefit of standards it introduces.

`Mint` avoids defining multiple aliasing types, so while there is no strongly typed protection, the user at least are not able to use one type in place of another. For this reason, there is no distinction between vectors and points here, as well as row-major versus column-major matrices. `Mint` attempts to be low-level and leave these semantical differences out of scope.
