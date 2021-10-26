# mint
[![Build Status](https://github.com/kvark/mint/workflows/check/badge.svg)](https://github.com/kvark/mint/actions)
[![Docs](https://docs.rs/mint/badge.svg)](https://docs.rs/mint)
[![Crates.io](https://img.shields.io/crates/v/mint.svg?maxAge=2592000)](https://crates.io/crates/mint)

**Math INteroperability Types**

![xkcd standard](https://imgs.xkcd.com/comics/standards.png)

This library provides standard mathematical types used in computer graphics.
Its only purpose is to serve as a standard and interoperability language between various components of [rust-gamedev](http://arewegameyet.com/categories/math/) ecosystem that happen to expose math-related types on their API.
There are no operations defined for the types other than for the means of conversion from/into external types.  
Serde support is available through the `serde` feature.

## Types

This crate offers the following types:

### Points

* [`Point2`](https://docs.rs/mint/*/mint/struct.Point2.html)
* [`Point3`](https://docs.rs/mint/*/mint/struct.Point3.html)

### Matrices

#### Column Matrices

* [`ColumnMatrix2`](https://docs.rs/mint/*/mint/struct.ColumnMatrix2.html)
* [`ColumnMatrix3`](https://docs.rs/mint/*/mint/struct.ColumnMatrix3.html)
* [`ColumnMatrix4`](https://docs.rs/mint/*/mint/struct.ColumnMatrix4.html)
* [`ColumnMatrix2x3`](https://docs.rs/mint/*/mint/struct.ColumnMatrix2x3.html)
* [`ColumnMatrix2x4`](https://docs.rs/mint/*/mint/struct.ColumnMatrix2x4.html)
* [`ColumnMatrix3x2`](https://docs.rs/mint/*/mint/struct.ColumnMatrix3x2.html)
* [`ColumnMatrix3x4`](https://docs.rs/mint/*/mint/struct.ColumnMatrix3x4.html)
* [`ColumnMatrix4x2`](https://docs.rs/mint/*/mint/struct.ColumnMatrix4x2.html)
* [`ColumnMatrix4x3`](https://docs.rs/mint/*/mint/struct.ColumnMatrix4x3.html)

#### Row Matrices

* [`RowMatrix2`](https://docs.rs/mint/*/mint/struct.RowMatrix2.html)
* [`RowMatrix3`](https://docs.rs/mint/*/mint/struct.RowMatrix3.html)
* [`RowMatrix4`](https://docs.rs/mint/*/mint/struct.RowMatrix4.html)
* [`RowMatrix2x3`](https://docs.rs/mint/*/mint/struct.RowMatrix2x3.html)
* [`RowMatrix2x4`](https://docs.rs/mint/*/mint/struct.RowMatrix2x4.html)
* [`RowMatrix3x2`](https://docs.rs/mint/*/mint/struct.RowMatrix3x2.html)
* [`RowMatrix3x4`](https://docs.rs/mint/*/mint/struct.RowMatrix3x4.html)
* [`RowMatrix4x2`](https://docs.rs/mint/*/mint/struct.RowMatrix4x2.html)
* [`RowMatrix4x3`](https://docs.rs/mint/*/mint/struct.RowMatrix4x3.html)

### Vectors

* [`Vector2`](https://docs.rs/mint/*/mint/struct.Vector2.html)
* [`Vector3`](https://docs.rs/mint/*/mint/struct.Vector3.html)
* [`Vector4`](https://docs.rs/mint/*/mint/struct.Vector4.html)

### Quaternion

* [`Quaternion`](https://docs.rs/mint/*/mint/struct.Quaternion.html)

### Euler Angles

* [`EulerAngles`](https://docs.rs/mint/*/mint/struct.EulerAngles.html)

## MSRV

mint supports Rust 1.52.1 and newer. From time to time, mint may increase the minimum supported Rust version in a minor version bump in order to take advantage of new Rust features.

## License

mint is available under the terms of the MIT license. See [LICENSE](LICENSE) or <https://opensource.org/licenses/MIT> for more information.