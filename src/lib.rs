/*!
Mint - Math interoperability standard types.

Defines basic math types useful for computer graphics via the built-in Rust types.
Designed to serve as an interoperability standard between libraries
that do not necessarily depend on `mint` directly.
*/

#![deny(missing_docs)]

/// Vector in 2D space.
pub type Vector2<T> = [T; 2];
/// Vector in 3D space.
pub type Vector3<T> = [T; 3];
/// Vector in 4D space.
/// Useful as a homogeneous 3D vector representation.
pub type Vector4<T> = [T; 4];

/// Standard quaternion.
/// Useful for representing rotation in 3D space.
pub type Quaternion<T> = (T, Vector3<T>);
/// Dual quaternion.
/// Useful for representing both translation and rotation,
/// because of better interpolation quality.
pub type DualQuaternion<T> = (Quaternion<T>, Quaternion<T>);

/// Row-major 2x2 matrix.
pub type RowMatrix2<T> = [[T; 2]; 2];
/// Row-major 3x2 matrix.
/// Useful for combinging rotation, scale, and translation in 2D space.
pub type RowMatrix3x2<T> = [[T; 3]; 2];
/// Row-major 3x3 matrix.
pub type RowMatrix3<T> = [[T; 3]; 3];
/// Row-major 4x3 matrix.
/// Useful for combinging rotation, scale, and translation in 3D space.
pub type RowMatrix4x3<T> = [[T; 4]; 3];
/// Row-major 4x4 matrix.
pub type RowMatrix4<T> = [[T; 4]; 4];

/// Column-major 2x2 matrix.
pub type ColMatrix2<T> = [[T; 2]; 2];
/// Column-major 2x3 matrix.
/// Useful for combinging rotation, scale, and translation in 2D space.
pub type ColMatrix2x3<T> = [[T; 2]; 3];
/// Column-major 3x3 matrix.
pub type ColMatrix3<T> = [[T; 3]; 3];
/// Column-major 3x4 matrix.
/// Useful for combinging rotation, scale, and translation in 3D space.
pub type ColMatrix3x4<T> = [[T; 3]; 4];
/// Column-major 4x4 matrix.
pub type ColMatrix4<T> = [[T; 4]; 4];
