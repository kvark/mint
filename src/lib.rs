/*!
Mint - Math interoperability standard types.

Defines basic math types useful for computer graphics.
Designed to serve as an interoperability standard between libraries.
*/
#![deny(missing_docs)]

mod rotation;
mod vector;

pub use rotation::{EulerAngles, Quaternion};
pub use vector::{Vector2, Vector3, Vector4};


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

/// A 3D transform represented by separate euler angles for rotation,
/// uniform scale, and a position vector.
pub type EulerScalePos<T> = (EulerAngles<T>, T, Vector3<T>);

/// Dual quaternion.
/// Useful for representing both translation and rotation,
/// because of better interpolation quality.
pub type DualQuaternion<T> = (Quaternion<T>, Quaternion<T>);
