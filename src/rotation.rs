use std::marker::PhantomData;
use vector::{Bivector, Vector3};


/// Standard quaternion represented by the scalar and vector parts.
/// Useful for representing rotation in 3D space.
/// Corresponds to a right-handed rotation matrix.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Quaternion<T> {
    /// Scalar part of a quaternion.
    pub s: T,
    /// Vector part of a quaternion.
    pub v: Vector3<T>,
}

impl<T: Clone> From<[T; 4]> for Quaternion<T> {
    fn from(v: [T; 4]) -> Self {
        Quaternion {
            s: v[3].clone(),
            v: Vector3 {
                x: v[0].clone(),
                y: v[1].clone(),
                z: v[2].clone(),
            },
        }
    }
}

impl<T> Into<[T; 4]> for Quaternion<T> {
    fn into(self) -> [T; 4] {
        [self.v.x, self.v.y, self.v.z, self.s]
    }
}

impl<T> AsRef<[T; 4]> for Quaternion<T> {
    fn as_ref(&self) -> &[T; 4] { unsafe { ::std::mem::transmute(self) } }
}


/// A rotor in 3D space represented by a scalar and bi-vector parts.
/// Similar to Quaternion, can equally (isomorphically) represent
/// rotations in 3D space.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Rotor<T> {
    /// Scalar part of a rotor.
    pub s: T,
    /// Vector part of a rotor.
    pub b: Bivector<T>,
}

impl<T: Clone> From<[T; 4]> for Rotor<T> {
    fn from(v: [T; 4]) -> Self {
        Rotor {
            s: v[0].clone(),
            b: Bivector {
                xy: v[1].clone(),
                xz: v[2].clone(),
                yz: v[3].clone(),
            },
        }
    }
}

impl<T> Into<[T; 4]> for Rotor<T> {
    fn into(self) -> [T; 4] {
        [self.s, self.b.xy, self.b.xz, self.b.yz]
    }
}

impl<T> AsRef<[T; 4]> for Rotor<T> {
    fn as_ref(&self) -> &[T; 4] { unsafe { ::std::mem::transmute(self) } }
}


/// Abstract set of Euler angles in 3D space. The basis of angles
/// is defined by the generic parameter `B`.
///
/// Note: there are multiple notations of Euler angles. They are
/// split in two groups:
///   - intrinsic (also known as "Tait-Bryan angles"): rotate around local axis
///   - extrinsic (also known as "Proper Euler angles"): rotate around world axis
/// For each interpretation, different axis may be chosen in different order.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct EulerAngles<T, B> {
    /// First angle of rotation in range [-pi, pi] (_pitch_).
    pub a: T,
    /// Second angle of rotation around in range [-pi/2, pi/2] (_yaw_).
    pub b: T,
    /// Third angle of rotation in range [-pi, pi] (_roll_).
    pub c: T,
    /// Marker for the phantom basis.
    pub marker: PhantomData<B>,
}

/// Intrinsic rotation around X, then Y, then Z axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum IntraXYZ {}
/// Intrinsic rotation around Z, then X, then Z axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum IntraZXZ {}
/// Intrinsic rotation around Z, then Y, then X axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum IntraZYX {}
/// Extrinsic rotation around X, then Y, then Z axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExtraXYZ {}
/// Extrinsic rotation around Z, then X, then Z axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExtraZXZ {}
/// Extrinsic rotation around Z, then Y, then X axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExtraZYX {}

impl<T: Clone, B> From<[T; 3]> for EulerAngles<T, B> {
    fn from(v: [T; 3]) -> Self {
        EulerAngles {
            a: v[0].clone(),
            b: v[1].clone(),
            c: v[2].clone(),
            marker: PhantomData,
        }
    }
}

impl<T, B> Into<[T; 3]> for EulerAngles<T, B> {
    fn into(self) -> [T; 3] {
        [self.a, self.b, self.c]
    }
}

macro_rules! reverse {
    ($from:ident -> $to:ident) => {
        impl<T> From<EulerAngles<T, $from>> for EulerAngles<T, $to> {
            fn from(other: EulerAngles<T, $from>) -> Self {
                EulerAngles {
                    a: other.c,
                    b: other.b,
                    c: other.a,
                    marker: PhantomData,
                }
            }
        }
    };
    ($from:ident <-> $to:ident) => {
        reverse!($from -> $to);
        reverse!($to -> $from);
    };
}

reverse!(IntraXYZ <-> ExtraZYX);
reverse!(IntraZXZ <-> ExtraZXZ);
reverse!(IntraZYX <-> ExtraXYZ);
