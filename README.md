# mint
[![Build Status](https://travis-ci.org/kvark/mint.svg)](https://travis-ci.org/kvark/mint)
[![Docs](https://docs.rs/mint/badge.svg)](https://docs.rs/mint)
[![Crates.io](https://img.shields.io/crates/v/mint.svg?maxAge=2592000)](https://crates.io/crates/mint)

Math Interoperability Types.

This library contains type aliases of common math types used in computer graphics, to Rust built-in types.
It's only purpose is to serve as a standard and interoperability language between various components of rust-gamedev ecosystem that happens to expose math-related types on their API.
Since there are no new types/traits here, users are not even required to depend on this library in order to take the benefit of standards it introduces.
