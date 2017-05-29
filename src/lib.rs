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

/// Euler rotation angles in 3D space.
pub type EulerAngles<T> = (T, T, T);
/// Standard quaternion represented by the scalar and vector parts.
/// Useful for representing rotation in 3D space.
pub type Quaternion<T> = (T, Vector3<T>);
/// Dual quaternion.
/// Useful for representing both translation and rotation,
/// because of better interpolation quality.
pub type DualQuaternion<T> = (Quaternion<T>, Quaternion<T>);

/// 2x2 matrix.
pub type Matrix2<T> = [[T; 2]; 2];
/// 2x3 matrix.
/// Useful for combinging rotation, scale, and translation in 2D space,
/// when stored as column-major.
pub type Matrix2x3<T> = [[T; 2]; 3];
/// 3x2 matrix.
/// Useful for combinging rotation, scale, and translation in 2D space,
/// when stored as row-major.
pub type Matrix3x2<T> = [[T; 3]; 2];
/// 3x3 matrix.
pub type Matrix3<T> = [[T; 3]; 3];
/// 3x4 matrix.
/// Useful for combinging rotation, scale, and translation in 3D space,
/// when stored as column major.
pub type Matrix3x4<T> = [[T; 3]; 4];
/// 4x3 matrix.
/// Useful for combinging rotation, scale, and translation in 3D space,
/// when stored as row major.
pub type Matrix4x3<T> = [[T; 4]; 3];
/// 4x4 matrix.
pub type Matrix4<T> = [[T; 4]; 4];

/// A 3D transform represented by separate rotation quaternion,
/// uniform scale, and a position vector.
pub type QuatScalePos<T> = (Quaternion<T>, T, Vector3<T>);
