use std::marker::PhantomData;
use vector::Vector3;


/// Standard quaternion represented by the scalar and vector parts.
/// Useful for representing rotation in 3D space.
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


/// Abstract set of Euler angles in 3D space. The basis of angles
/// is defined by the generic parameter `B`.
///
/// Note: there are multiple notations of Euler angles. They are
/// split in two groups:
///   - intrinsic (also known as "Tait-Bryan angles"): rotate around local axis
///   - extrinsic (also known as "Proper Euler angles"): rotate around world axis
/// For each interpretation, different axis may be choosen in different order.
#[repr(C)]
pub struct EuluerAngles<T, B> {
    /// First angle of rotation in range [-pi, pi] (_pitch_).
    pub a: T,
    /// Second angle of rotation around in range [-pi/2, pi/2] (_yaw_).
    pub b: T,
    /// Third angle of rotation in range [-pi, pi] (_roll_).
    pub c: T,
    /// Marker for the phantom basis.
    pub marker: PhantomData<B>,
}

impl<T: Clone, B> From<[T; 3]> for EuluerAngles<T, B> {
    fn from(v: [T; 3]) -> Self {
        EuluerAngles {
            a: v[0].clone(),
            b: v[1].clone(),
            c: v[2].clone(),
            marker: PhantomData,
        }
    }
}

impl<T, B> Into<[T; 3]> for EuluerAngles<T, B> {
    fn into(self) -> [T; 3] {
        [self.a, self.b, self.c]
    }
}

/// Intrinsic rotation around X, then Y, then Z axis.
pub enum IntraXYZ {}
/// Intrinsic rotation around Z, then X, then Z axis.
pub enum IntraZXZ {}
/// Intrinsic rotation around Z, then Y, then X axis.
pub enum IntraZYX {}
/// Extrinsic rotation around X, then Y, then Z axis.
pub enum ExtraXYZ {}
/// Extrinsic rotation around Z, then X, then Z axis.
pub enum ExtraZXZ {}
/// Extrinsic rotation around Z, then Y, then X axis.
pub enum ExtraZYX {}

macro_rules! reverse {
    ($from:ident -> $to:ident) => {
        impl<T> From<EuluerAngles<T, $from>> for EuluerAngles<T, $to> {
            fn from(other: EuluerAngles<T, $from>) -> Self {
                EuluerAngles {
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
